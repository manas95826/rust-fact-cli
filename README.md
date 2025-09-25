# Fact CLI 🚀

A fun and educational CLI tool that fetches random coding, AI, and scaling facts from real APIs! Perfect for learning Rust while building something actually usable. Can also send facts to Telegram every 6 hours!

## Features

- 🌐 **Real API Facts**: Fetches facts from multiple real APIs (Useless Facts, Chuck Norris, Cat Facts, and more)
- 🏷️ **Categories**: Filter facts by category (coding, AI, scaling, tech, programming, or all)
- 🔢 **Multiple Facts**: Display multiple facts at once
- 🎨 **Colorful Output**: Beautiful, colored terminal output with emojis
- ⏰ **Watch Mode**: Continuous fact display for learning sessions
- 🤖 **Telegram Bot**: Send facts to Telegram every 6 hours automatically
- 🚀 **Fast**: Built with Rust for optimal performance
- 🔄 **Async**: Uses async/await for efficient API calls

## Installation

### Prerequisites
- Rust (install from [rustup.rs](https://rustup.rs/))

### Build from Source
```bash
git clone <your-repo-url>
cd fact-cli
cargo build --release
```

The binary will be available at `target/release/fact-cli`.

## Setup

### 1. Telegram Bot Setup (Optional)
If you want to use the Telegram bot feature:

1. **Create a Telegram Bot**:
   - Open Telegram and search for [@BotFather](https://t.me/BotFather)
   - Send `/newbot` and follow the instructions
   - Save the bot token you receive

2. **Get Your Chat ID**:
   - Start a chat with your bot
   - Send any message to the bot
   - Visit: `https://api.telegram.org/bot<YOUR_BOT_TOKEN>/getUpdates`
   - Find your chat ID in the response

3. **Set Environment Variables**:
   ```bash
   # Copy the example file
   cp .env.example .env
   
   # Edit .env with your credentials
   TELEGRAM_BOT_TOKEN=your_bot_token_here
   TELEGRAM_CHAT_ID=your_chat_id_here
   ```

## Usage

### Basic Usage
```bash
# Get a random fact from APIs
./target/release/fact-cli

# Get 3 random facts
./target/release/fact-cli --count 3

# Get facts from a specific category
./target/release/fact-cli --category coding
./target/release/fact-cli --category ai
./target/release/fact-cli --category scaling
```

### Advanced Usage
```bash
# Get 5 coding facts
./target/release/fact-cli --category coding --count 5

# Watch mode - continuous facts (press Enter for next batch)
./target/release/fact-cli --watch

# Watch mode with specific category
./target/release/fact-cli --category ai --watch --count 2

# Start Telegram bot (sends facts every 6 hours)
./target/release/fact-cli --telegram --category ai
```

### Command Line Options
- `-c, --count <COUNT>`: Number of facts to display (default: 1)
- `-t, --category <CATEGORY>`: Category of facts (all, coding, ai, scaling, tech, programming)
- `-w, --watch`: Display facts continuously
- `-b, --telegram`: Start Telegram bot mode (sends facts every 6 hours)
- `-h, --help`: Show help information
- `-V, --version`: Show version information

## Examples

```bash
# Quick coding fact
./target/release/fact-cli --category coding

# Learning session with AI facts
./target/release/fact-cli --category ai --count 5

# Continuous scaling facts for study
./target/release/fact-cli --category scaling --watch
```

## Categories

- **🌟 All**: Mix of facts from all APIs and categories
- **💻 Coding**: Programming languages, history, and development facts
- **🤖 AI**: Artificial intelligence, machine learning, and neural networks
- **📈 Scaling**: System architecture, distributed systems, and performance
- **⚡ Tech**: General technology and computing facts
- **💻 Programming**: Programming-specific facts and trivia

## APIs Used

- **Useless Facts API**: Random interesting facts
- **Chuck Norris API**: Programming and tech-related jokes
- **Cat Facts API**: Fun animal facts
- **Custom Fact Databases**: Curated coding, AI, and scaling facts

## Development

### Running in Development
```bash
cargo run
cargo run -- --category coding --count 3
```

### Adding New Facts
Edit the `FactDatabase` struct in `src/main.rs` to add new facts to any category.

### Dependencies
- `clap`: Command-line argument parsing
- `colored`: Terminal colors and styling
- `rand`: Random number generation
- `reqwest`: HTTP client for API requests
- `serde`: JSON serialization/deserialization
- `tokio`: Async runtime
- `teloxide`: Telegram bot framework
- `dotenv`: Environment variable loading
- `chrono`: Date and time handling

## License

MIT License - feel free to use and modify!

---

**Happy Learning!** 🎉 This tool is perfect for:
- Quick learning breaks
- Terminal fun
- Studying system design
- Understanding programming history
- Exploring AI concepts
