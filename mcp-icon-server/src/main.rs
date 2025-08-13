use anyhow::Result;
use rmcp::{ServiceExt, transport::stdio};
use tracing_subscriber::EnvFilter;

mod icon_index;
mod server;
mod tools;
mod types;

use server::IconServer;

#[tokio::main]
async fn main() -> Result<()> {
    // Initialize tracing for debugging (logs to stderr)
    tracing_subscriber::fmt()
        .with_env_filter(
            EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| EnvFilter::new("warn")),
        )
        .with_writer(std::io::stderr)
        .init();

    tracing::info!("Starting MCP Icon Server for yew-shortcuts");

    // Create the server instance
    let service = IconServer::new()?;

    // Serve over stdio transport for Claude Code
    let transport = stdio();
    
    tracing::info!("Server initialized, waiting for connections...");
    
    let server = service.serve(transport).await?;
    
    // Wait for the server to complete
    server.waiting().await?;

    Ok(())
}