#!/bin/bash

echo "🚀 Fact CLI Deployment Helper"
echo "============================="
echo ""

# Check if we're in the right directory
if [ ! -f "Cargo.toml" ]; then
    echo "❌ Please run this script from the fact-cli directory"
    exit 1
fi

echo "📋 Deployment Options:"
echo ""
echo "1. 🖥️  Local Background Process (Free)"
echo "   - Runs on your computer"
echo "   - Stops when computer shuts down"
echo "   - Good for testing"
echo ""
echo "2. ☁️  Cloud VPS (Paid - $5/month)"
echo "   - Runs 24/7 on cloud server"
echo "   - More reliable"
echo "   - Requires server setup"
echo ""
echo "3. 🆓 Free Cloud Services"
echo "   - Heroku (free tier)"
echo "   - Railway (free tier)"
echo "   - Render (free tier)"
echo ""

read -p "Choose option (1-3): " choice

case $choice in
    1)
        echo ""
        echo "🖥️  Setting up Local Background Process..."
        echo ""
        
        # Check if .env exists
        if [ ! -f ".env" ]; then
            echo "❌ .env file not found. Please set up your Telegram credentials first."
            exit 1
        fi
        
        # Build the project
        echo "🔨 Building project..."
        cargo build --release
        
        if [ $? -ne 0 ]; then
            echo "❌ Build failed!"
            exit 1
        fi
        
        echo "✅ Build successful!"
        echo ""
        
        # Create a systemd service (Linux/Mac)
        if command -v systemctl &> /dev/null; then
            echo "📝 Creating systemd service..."
            sudo tee /etc/systemd/system/fact-cli.service > /dev/null <<EOF
[Unit]
Description=Fact CLI Telegram Bot
After=network.target

[Service]
Type=simple
User=$USER
WorkingDirectory=$(pwd)
EnvironmentFile=$(pwd)/.env
ExecStart=$(pwd)/target/release/fact-cli --telegram --category all
Restart=always
RestartSec=10

[Install]
WantedBy=multi-user.target
EOF
            
            echo "✅ Systemd service created!"
            echo ""
            echo "🔧 To start the service:"
            echo "   sudo systemctl start fact-cli"
            echo "   sudo systemctl enable fact-cli"
            echo ""
            echo "🔍 To check status:"
            echo "   sudo systemctl status fact-cli"
            echo ""
            echo "🛑 To stop:"
            echo "   sudo systemctl stop fact-cli"
        else
            echo "📝 Starting background process..."
            nohup ./target/release/fact-cli --telegram --category all > bot.log 2>&1 &
            echo "✅ Bot started in background!"
            echo ""
            echo "🔍 To check logs:"
            echo "   tail -f bot.log"
            echo ""
            echo "🛑 To stop:"
            echo "   pkill fact-cli"
        fi
        ;;
        
    2)
        echo ""
        echo "☁️  Cloud VPS Deployment..."
        echo ""
        echo "📋 Steps:"
        echo "1. Create a VPS (DigitalOcean, AWS, etc.)"
        echo "2. Install Rust: curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh"
        echo "3. Upload this project to the server"
        echo "4. Set up environment variables"
        echo "5. Run the bot"
        echo ""
        echo "💡 Need help? Check the README.md for detailed instructions"
        ;;
        
    3)
        echo ""
        echo "🆓 Free Cloud Services..."
        echo ""
        echo "📋 Recommended: Railway.app"
        echo "1. Go to https://railway.app"
        echo "2. Connect your GitHub account"
        echo "3. Upload this project"
        echo "4. Set environment variables in Railway dashboard"
        echo "5. Deploy!"
        echo ""
        echo "💡 The bot will run 24/7 for free!"
        ;;
        
    *)
        echo "❌ Invalid option"
        exit 1
        ;;
esac

echo ""
echo "🎉 Deployment setup complete!"
echo ""
echo "📱 Your bot will send facts every 6 hours to your Telegram!"
