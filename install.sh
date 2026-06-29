#!/bin/bash
set -e

echo "🔨 Building Smart Shell in release mode..."
cargo build --release

BIN_NAME="smartsh"
INSTALL_DIR="$HOME/.local/bin"
CONFIG_DIR="$HOME/.config/smartsh"

echo "📂 Setting up configuration directory at $CONFIG_DIR..."
mkdir -p "$CONFIG_DIR"

if [ ! -f "$CONFIG_DIR/.env" ]; then
    echo "API_KEY=\"ak_2PM1wL5y006j9hQ9Zp2E84MR4yX5e\"" > "$CONFIG_DIR/.env"
    echo "BASE_URL=\"https://api.longcat.chat/openai/chat/completions\"" >> "$CONFIG_DIR/.env"
    echo "MODEL=\"LongCat-2.0-Preview\"" >> "$CONFIG_DIR/.env"
    echo "✅ Created default configuration at $CONFIG_DIR/.env"
else
    echo "ℹ️  Configuration already exists at $CONFIG_DIR/.env"
fi

echo "📦 Installing binary to $INSTALL_DIR/$BIN_NAME..."
mkdir -p "$INSTALL_DIR"
cp target/release/smart_shell "$INSTALL_DIR/$BIN_NAME"

echo ""
echo "🎉 Installation complete!"
echo "Make sure $INSTALL_DIR is in your system PATH."
echo "You can now run '$BIN_NAME' from anywhere in your terminal!"
