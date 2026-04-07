#!/bin/bash
set -e
REPO="fusiontech21/Cora-for-coraos"

# Clean up the JSON to get just the version number
LATEST=$(curl -s https://api.github.com/repos/$REPO/releases/latest | grep '"tag_name"' | head -1 | tr -d ' ",')

if [[ -z "$LATEST" ]]; then
    echo "Error: Could not find version."
    exit 1
fi

BIN_URL="https://github.com/$REPO/releases/download/$LATEST/cora"

echo "Downloading Cora $LATEST..."
curl -fSL "$BIN_URL" -o /tmp/cora-new

sudo install -m 755 /tmp/cora-new /usr/local/bin/cora
rm /tmp/cora-new
echo "Cora updated to $LATEST"
