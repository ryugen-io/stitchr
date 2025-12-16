#!/bin/bash
set -e

# Sweet Dracula Theme Colors (TrueColor)
# Hex: #BD93F9 (Purple)
COLOR_PRIMARY='\033[38;2;189;147;249m'
# Hex: #FF79C6 (Pink)
COLOR_SECONDARY='\033[38;2;255;121;198m'
# Hex: #8BE9FD (Cyan)
COLOR_INFO='\033[38;2;139;233;253m'
# Hex: #50FA7B (Green)
COLOR_SUCCESS='\033[38;2;80;250;123m'
# Hex: #FF5555 (Red)
COLOR_ERROR='\033[38;2;255;85;85m'
# Reset
NC='\033[0m'

# Ascii Art with Gradient Effect (Line by Line)
echo -e "${COLOR_PRIMARY} _______ _______ _____ _______ _______ _     _  ______${NC}"
echo -e "${COLOR_SECONDARY} |______    |      |      |    |       |_____| |_____/${NC}"
echo -e "${COLOR_INFO} ______|    |    __|__    |    |_____  |     | |    \_${NC}"
echo ""

echo -e "${COLOR_PRIMARY}==> Building 'stitchr' (Release mode)...${NC}"

# Ensure we are in the project root
if [ ! -f "Cargo.toml" ]; then
    echo -e "${COLOR_ERROR}Error: Cargo.toml not found. Please run this script from the project root.${NC}"
    exit 1
fi

# Build the project
cargo build --release -p stitchr-cli

# Verify build success
BINARY_PATH="target/release/stitchr"
if [ ! -f "$BINARY_PATH" ]; then
    echo -e "${COLOR_ERROR}Error: Binary not found at $BINARY_PATH after build.${NC}"
    exit 1
fi

echo -e "${COLOR_SUCCESS}==> Build successful!${NC}"

# Determine install location
INSTALL_DIR="$HOME/.local/bin"
if [ ! -d "$INSTALL_DIR" ]; then
    echo -e "${COLOR_INFO}==> '$INSTALL_DIR' does not exist. Checking '$HOME/bin'...${NC}"
    INSTALL_DIR="$HOME/bin"
    if [ ! -d "$INSTALL_DIR" ]; then
        echo -e "${COLOR_INFO}==> Creating '$HOME/.local/bin'...${NC}"
        INSTALL_DIR="$HOME/.local/bin"
        mkdir -p "$INSTALL_DIR"
    fi
fi

echo -e "${COLOR_INFO}==> Installing to '$INSTALL_DIR'...${NC}"

# Copy binary
cp "$BINARY_PATH" "$INSTALL_DIR/stitchr"

echo -e "${COLOR_SUCCESS}==> Installed 'stitchr' to $INSTALL_DIR/stitchr${NC}"

# Check PATH
if ! echo ":$PATH:" | grep -q ":$INSTALL_DIR:"; then
    echo -e "${COLOR_ERROR}Warning: $INSTALL_DIR is not in your PATH.${NC}"
    echo "You may need to add the following line to your shell configuration (.bashrc, .zshrc, etc.):"
    echo "  export PATH=\"\$PATH:$INSTALL_DIR\""
else
    echo -e "${COLOR_SUCCESS}==> Installation complete! Run 'stitchr --help' to get started.${NC}"
fi