use anyhow::Result;
use rmcp::{
    ErrorData as McpError, RoleServer, ServerHandler,
    model::*,
    service::RequestContext,
};
use serde_json::{json, Map, Value};

use crate::icon_index::IconIndex;
use crate::tools;
use crate::types::*;

#[derive(Clone)]
pub struct IconServer {
    index: IconIndex,
}

impl IconServer {
    pub fn new() -> Result<Self> {
        let index = IconIndex::load()?;
        Ok(Self { index })
    }
}

impl ServerHandler for IconServer {
    fn get_info(&self) -> ServerInfo {
        ServerInfo::new(ServerCapabilities::builder().enable_tools().build())
            .with_protocol_version(ProtocolVersion::V_2024_11_05)
            .with_server_info(Implementation::from_build_env())
            .with_instructions(
                "This server provides FontAwesome icon search and code generation for yew-shortcuts. \
                Use 'search_icons' to find icons, 'get_icon_code' to generate Yew component code, \
                'get_icon_details' for detailed icon information, and 'list_categories' to see available categories."
            )
    }

    async fn list_tools(
        &self,
        _request: Option<PaginatedRequestParams>,
        _context: RequestContext<RoleServer>,
    ) -> Result<ListToolsResult, McpError> {
        let mut search_schema = Map::new();
        search_schema.insert("type".to_string(), json!("object"));
        search_schema.insert("properties".to_string(), json!({
            "query": {
                "type": "string",
                "description": "Search query for icon names"
            },
            "category": {
                "type": "string",
                "enum": ["solid", "regular", "brands"],
                "description": "Optional category filter"
            }
        }));
        search_schema.insert("required".to_string(), json!(["query"]));

        let mut code_schema = Map::new();
        code_schema.insert("type".to_string(), json!("object"));
        code_schema.insert("properties".to_string(), json!({
            "name": {
                "type": "string",
                "description": "Icon name (e.g., 'house', 'user')"
            },
            "category": {
                "type": "string",
                "enum": ["solid", "regular", "brands"],
                "description": "Icon category"
            }
        }));
        code_schema.insert("required".to_string(), json!(["name", "category"]));

        let mut details_schema = Map::new();
        details_schema.insert("type".to_string(), json!("object"));
        details_schema.insert("properties".to_string(), json!({
            "name": {
                "type": "string",
                "description": "Icon name"
            },
            "category": {
                "type": "string",
                "enum": ["solid", "regular", "brands"],
                "description": "Icon category"
            }
        }));
        details_schema.insert("required".to_string(), json!(["name", "category"]));

        let mut categories_schema = Map::new();
        categories_schema.insert("type".to_string(), json!("object"));
        categories_schema.insert("properties".to_string(), json!({}));

        Ok(ListToolsResult::with_all_items(vec![
            Tool::new("search_icons", "Search for FontAwesome icons by name or keyword", search_schema),
            Tool::new("get_icon_code", "Get Yew component code for a specific icon", code_schema),
            Tool::new("get_icon_details", "Get detailed information about a specific icon", details_schema),
            Tool::new("list_categories", "List all available icon categories with counts", categories_schema),
        ]))
    }

    async fn call_tool(
        &self,
        CallToolRequestParams { name, arguments, .. }: CallToolRequestParams,
        _context: RequestContext<RoleServer>,
    ) -> Result<CallToolResult, McpError> {
        // Convert Option<Map> to Value for deserialization
        let arguments = arguments.map(Value::Object).unwrap_or(Value::Null);
        
        let result = match name.as_ref() {
            "search_icons" => {
                let params: SearchIconsParams = serde_json::from_value(arguments)
                    .map_err(|e| McpError::invalid_params("Invalid parameters", Some(json!({"error": e.to_string()}))))?;
                let result = tools::search::search_icons(&self.index, params)
                    .map_err(|e| McpError::internal_error("Search failed", Some(json!({"error": e.to_string()}))))?;
                serde_json::to_value(result)
                    .map_err(|e| McpError::internal_error("Serialization failed", Some(json!({"error": e.to_string()}))))?
            }
            "get_icon_code" => {
                let params: GetIconCodeParams = serde_json::from_value(arguments)
                    .map_err(|e| McpError::invalid_params("Invalid parameters", Some(json!({"error": e.to_string()}))))?;
                let result = tools::code_gen::get_icon_code(&self.index, params)
                    .map_err(|e| McpError::internal_error("Code generation failed", Some(json!({"error": e.to_string()}))))?;
                serde_json::to_value(result)
                    .map_err(|e| McpError::internal_error("Serialization failed", Some(json!({"error": e.to_string()}))))?
            }
            "get_icon_details" => {
                let params: GetIconDetailsParams = serde_json::from_value(arguments)
                    .map_err(|e| McpError::invalid_params("Invalid parameters", Some(json!({"error": e.to_string()}))))?;
                let result = tools::details::get_icon_details(&self.index, params)
                    .map_err(|e| McpError::internal_error("Details retrieval failed", Some(json!({"error": e.to_string()}))))?;
                serde_json::to_value(result)
                    .map_err(|e| McpError::internal_error("Serialization failed", Some(json!({"error": e.to_string()}))))?
            }
            "list_categories" => {
                let result = tools::list_categories(&self.index);
                serde_json::to_value(result)
                    .map_err(|e| McpError::internal_error("Serialization failed", Some(json!({"error": e.to_string()}))))?
            }
            _ => {
                return Err(McpError::method_not_found::<PingRequestMethod>());
            }
        };

        Ok(CallToolResult::success(vec![Content::text(
            result.to_string(),
        )]))
    }
}