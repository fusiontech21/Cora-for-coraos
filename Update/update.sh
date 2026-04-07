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
TMPFILE=$(mktemp /tmp/cora-new.XXXXXX)
curl -fSL "$URL" -o "$TMPFILE"

# Verify it's actually a valid binary before installing
if ! file "$TMPFILE" | grep -q "ELF"; then
    echo "Error: Downloaded file is not a valid binary."
    rm -f "$TMPFILE"
    exit 1
fi

sudo install -m 755 "$TMPFILE" /usr/local/bin/cora
rm -f "$TMPFILE"
echo "Update complete."
