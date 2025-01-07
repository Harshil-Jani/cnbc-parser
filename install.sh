#!/bin/bash

# Define variables
VERSION="v0.1.0"
BINARY_NAME="cnbc-parser"
GITHUB_URL_BASE="https://github.com/Harshil-Jani/cnbc-parser/releases/download/$VERSION"

# Detect the operating system
OS=$(uname -s)

if [ "$OS" == "Darwin" ]; then
    BINARY_NAME="${BINARY_NAME}_mac"
elif [ "$OS" == "Linux" ]; then
    BINARY_NAME="${BINARY_NAME}_linux"
else
    echo "Unsupported OS: $OS"
    exit 1
fi

GITHUB_URL="$GITHUB_URL_BASE/$BINARY_NAME"

# Download the binary
echo "Downloading $BINARY_NAME..."
curl -sSL "$GITHUB_URL" -o "$BINARY_NAME"

# Make the binary executable
chmod +x "$BINARY_NAME"

# Move the binary to /usr/local/bin for global usage
echo "Installing $BINARY_NAME to /usr/local/bin..."
sudo mv "$BINARY_NAME" /usr/local/bin/cnbc-parser

# Verify the installation
if command -v cnbc-parser >/dev/null 2>&1; then
    echo "cnbc-parser has been installed successfully!"
else
    echo "Installation failed. Please try again. If the issue persists, please open an issue at https://github.com/Harshil-Jani/cnbc-parser/issues"
fi