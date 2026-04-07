#!/bin/bash
set -e
REPO="fusiontech21/Cora-for-coraos"

echo "Fetching update info..."

URL=$(curl -s "https://api.github.com/repos/$REPO/releases/latest" \
    | grep "browser_download_url" \
    | grep "cora" \
    | head -1 \
    | sed -n 's/.*"browser_download_url": *"\([^"]*\)".*/\1/p')

if [ -z "$URL" ]; then
    echo "Error: Could not find the download link."
    exit 1
fi

echo "Downloading from: $URL"
curl -fSL "$URL" -o /tmp/cora-new

sudo install -m 755 /tmp/cora-new /usr/local/bin/cora
rm -f /tmp/cora-new
echo "Update complete."
