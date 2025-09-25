#!/bin/bash

echo "🚀 Setting up Fact CLI..."

# Check if Rust is installed
if ! command -v cargo &> /dev/null; then
    echo "❌ Rust is not installed. Please install Rust from https://rustup.rs/"
    exit 1
fi

echo "✅ Rust is installed"

# Build the project
echo "🔨 Building the project..."
cargo build --release

if [ $? -eq 0 ]; then
    echo "✅ Build successful!"
else
    echo "❌ Build failed!"
    exit 1
fi

# Create .env file if it doesn't exist
if [ ! -f .env ]; then
    echo "📝 Creating .env file from template..."
    cp .env.example .env
    echo "⚠️  Please edit .env file with your Telegram bot credentials"
else
    echo "✅ .env file already exists"
fi

echo ""
echo "🎉 Setup complete!"
echo ""
echo "Usage examples:"
echo "  ./target/release/fact-cli                           # Get a random fact"
echo "  ./target/release/fact-cli --category ai --count 3  # Get 3 AI facts"
echo "  ./target/release/fact-cli --watch                   # Watch mode"
echo "  ./target/release/fact-cli --telegram --category ai # Telegram bot mode"
echo ""
echo "For Telegram bot mode, make sure to:"
echo "1. Create a bot with @BotFather on Telegram"
echo "2. Get your chat ID"
echo "3. Update the .env file with your credentials"
