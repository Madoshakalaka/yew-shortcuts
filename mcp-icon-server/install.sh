#!/bin/bash

set -e

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
NC='\033[0m' # No Color

echo -e "${GREEN}Installing MCP Icon Server for yew-shortcuts...${NC}"

# Check if cargo is installed
if ! command -v cargo &> /dev/null; then
    echo -e "${RED}Error: cargo is not installed. Please install Rust first.${NC}"
    exit 1
fi


# Get the script directory
SCRIPT_DIR="$( cd "$( dirname "${BASH_SOURCE[0]}" )" && pwd )"
PROJECT_ROOT="$(dirname "$SCRIPT_DIR")"

echo -e "${GREEN}Building MCP server in release mode...${NC}"
cd "$PROJECT_ROOT"
cargo build --release --package mcp-icon-server --quiet

# Create binary directory if it doesn't exist
INSTALL_DIR="/usr/local/bin"
if [ ! -w "$INSTALL_DIR" ]; then
    echo -e "${YELLOW}Need sudo permission to install to $INSTALL_DIR${NC}"
    sudo mkdir -p "$INSTALL_DIR"
fi

# Copy the binary
BINARY_NAME="mcp-icon-server"
BINARY_SOURCE="$PROJECT_ROOT/target/release/$BINARY_NAME"
BINARY_DEST="$INSTALL_DIR/$BINARY_NAME"

if [ -f "$BINARY_SOURCE" ]; then
    echo -e "${GREEN}Installing binary to $BINARY_DEST...${NC}"
    if [ -w "$INSTALL_DIR" ]; then
        cp "$BINARY_SOURCE" "$BINARY_DEST"
    else
        sudo cp "$BINARY_SOURCE" "$BINARY_DEST"
    fi
    if [ -w "$BINARY_DEST" ]; then
        chmod +x "$BINARY_DEST"
    else
        sudo chmod +x "$BINARY_DEST"
    fi
else
    echo -e "${RED}Error: Binary not found at $BINARY_SOURCE${NC}"
    exit 1
fi

# Test that the binary runs
echo -e "${GREEN}Testing MCP server binary...${NC}"
if timeout 1s "$BINARY_DEST" --help &>/dev/null || [ $? -eq 124 ]; then
    echo -e "${GREEN}Binary installed successfully!${NC}"
else
    echo -e "${YELLOW}Warning: Binary may not be working correctly${NC}"
fi


echo -e "${GREEN}========================================${NC}"
echo -e "${GREEN}Installation complete!${NC}"
echo -e "${GREEN}========================================${NC}"
echo
echo -e "The MCP Icon Server has been installed successfully."
echo
echo -e "To add this MCP server to Claude Code, run:"
echo
echo -e "  ${YELLOW}claude mcp add yew-shortcuts-icons $BINARY_DEST -e RUST_LOG=warn${NC}"
echo
echo -e "After adding, the server will provide these tools:"
echo -e "   - search_icons: Search for FontAwesome icons"
echo -e "   - get_icon_code: Get Yew component code for an icon"
echo -e "   - get_icon_details: Get detailed icon information"
echo -e "   - list_categories: List available icon categories"