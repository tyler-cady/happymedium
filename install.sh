#!/bin/sh

set -e

REPO="tyler-cady/happymedium"
VERSION=$(curl -s "https://api.github.com/repos/$REPO/releases/latest" | sed -n 's/.*"tag_name": "\([^"]*\)".*/\1/p')

OS="macos"  # Always set for macOS

ARCH=$(uname -m)

case "$ARCH" in
    x86_64) ARCH="x86_64" ;;
    aarch64|arm64) ARCH="aarch64" ;;  # Apple M1/M2
    *) echo "Unsupported architecture: $ARCH"; exit 1 ;;
esac

BINARY="happymedium-${OS}-${ARCH}"

URL="https://github.com/$REPO/releases/download/$VERSION/$BINARY"

echo "Downloading Happy Medium from $URL..."
curl -L "$URL" -o "/tmp/happymedium"

# Check if the file was downloaded successfully
if [ ! -f "/tmp/happymedium" ]; then
    echo "Download failed. Exiting."
    exit 1
fi

chmod +x "/tmp/happymedium"

# Move the binary to the appropriate location for macOS
sudo mv "/tmp/happymedium" /usr/local/bin/happymedium
echo "Installation complete! Run 'happymedium' to start."
