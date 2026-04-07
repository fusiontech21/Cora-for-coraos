#!/bin/bash
set -e
REPO="fusiontech21/Cora-for-coraos"
LATEST=$(curl -fsSL "https://api.github.com/repos/$REPO/releases/latest" | grep -m1 tag_name | grep -oP '[\d.]+')
BIN_URL="https://github.com/$REPO/releases/download/$LATEST/cora"
echo "Downloading Cora $LATEST..."
curl -fSL "$BIN_URL" -o /tmp/cora-new
sudo install -m 755 /tmp/cora-new /usr/local/bin/cora
rm -f /tmp/cora-new
echo "Cora updated to $LATEST"
