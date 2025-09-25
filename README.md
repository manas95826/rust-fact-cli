# Fact CLI 🚀

A powerful CLI tool that fetches random coding, AI, and scaling facts from real APIs and can send them to Telegram every 6 hours! Built with Rust for maximum performance and reliability.

## 🌟 What Makes This Special?

### **Real-Time Data from Multiple Sources**
- **Live APIs**: Fetches fresh facts from real internet APIs
- **Smart Fallbacks**: Uses curated local facts when APIs are down
- **Diverse Content**: Mix of programming, AI, scaling, and general tech facts
- **Always Available**: Never fails to deliver facts, even offline

### **Built with Rust for Production Quality**
- **Lightning Fast**: Compiled to native code, runs as fast as C++
- **Memory Safe**: Zero crashes, no memory leaks, no segfaults
- **Concurrent**: Handles multiple API calls efficiently with async/await
- **Single Binary**: No dependencies, easy to distribute and run

## 🎯 Features

- 🌐 **Real API Facts**: Fetches from Useless Facts, Chuck Norris, Cat Facts APIs
- 🏷️ **Smart Categories**: Filter by coding, AI, scaling, tech, or programming
- 🔢 **Batch Mode**: Get multiple facts at once
- 🎨 **Beautiful Output**: Colorful terminal display with emojis
- ⏰ **Watch Mode**: Continuous fact streaming for learning
- 🤖 **Telegram Bot**: Automated fact delivery every 6 hours
- 🚀 **High Performance**: Built with Rust for speed and reliability
- 🔄 **Async Operations**: Non-blocking API calls and processing

## Installation

### Prerequisites
- Rust (install from [rustup.rs](https://rustup.rs/))

### Build from Source
```bash
git clone https://github.com/manas95826/rust-fact-cli.git
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

## 📚 Fact Sources & Data Flow

### **Live APIs (Primary Sources)**
Our tool fetches fresh facts from real internet APIs:

#### **1. Useless Facts API** 🌟
- **URL**: `https://uselessfacts.jsph.pl/random.json`
- **Content**: Random interesting facts about anything
- **Example**: "The human brain contains approximately 86 billion neurons"
- **Why**: Provides diverse, educational content

#### **2. Chuck Norris API** 💪
- **URL**: `https://api.chucknorris.io/jokes/random`
- **Content**: Programming and tech-related jokes
- **Example**: "Chuck Norris doesn't need garbage collection because he doesn't call .Dispose(), he calls .DropKick()"
- **Why**: Fun, programming-themed humor

#### **3. Cat Facts API** 🐱
- **URL**: `https://cat-fact.herokuapp.com/facts/random`
- **Content**: Animal and nature facts
- **Example**: "Cats have a third eyelid called a nictitating membrane"
- **Why**: Adds variety and light-hearted content

### **Local Fallback Database (Backup Sources)**
When APIs are down, we use curated facts:

#### **Coding Facts** 💻
- **15+ facts** about programming languages, history, and development
- **Examples**: "JavaScript was created in just 10 days", "The first computer bug was an actual bug"
- **Source**: Hand-picked from programming history and trivia

#### **AI Facts** 🤖
- **15+ facts** about artificial intelligence and machine learning
- **Examples**: "GPT-3 has 175 billion parameters", "The Turing Test was proposed in 1950"
- **Source**: Curated from AI research and history

#### **Scaling Facts** 📈
- **15+ facts** about system architecture and distributed systems
- **Examples**: "Google processes 8.5 billion searches per day", "The CAP theorem states..."
- **Source**: System design principles and real-world examples

### **Smart Fallback System**
```rust
// Try APIs first, fallback to local facts
if let Ok(fact) = self.fetch_useless_fact().await {
    return Ok(fact);
}
if let Ok(fact) = self.fetch_chuck_norris_fact().await {
    return Ok(fact);
}
// ... more API attempts
// If all APIs fail, use local curated facts
self.fetch_tech_fact().await
```

**Why This Approach?**
- ✅ **Always Works**: Never fails to deliver facts
- ✅ **Fresh Content**: Gets new facts from the internet
- ✅ **Reliable**: Falls back to curated content when needed
- ✅ **Diverse**: Mix of live and curated content

## 🦀 Why Rust for This Project?

### **Perfect for Long-Running Processes**
Our Telegram bot runs 24/7, fetching facts every 6 hours. Rust ensures:
- **Zero Memory Leaks**: Won't consume more memory over time
- **No Crashes**: Memory safety prevents segfaults and panics
- **High Performance**: Compiled to native code, runs as fast as C++
- **Efficient Resource Usage**: Low CPU and memory footprint

### **Excellent Concurrency for API Calls**
```rust
// Rust handles multiple API calls efficiently
async fn fetch_random_fact(&self, category: &Category) -> Result<String, Box<dyn std::error::Error>> {
    // Try multiple APIs concurrently
    if let Ok(fact) = self.fetch_useless_fact().await {
        return Ok(fact);
    }
    if let Ok(fact) = self.fetch_chuck_norris_fact().await {
        return Ok(fact);
    }
    // Fallback to local facts
    self.fetch_tech_fact().await
}
```

**Why This Matters:**
- **Non-blocking I/O**: Can handle multiple API requests simultaneously
- **Lightweight Threads**: Thousands of concurrent operations
- **Perfect for Network-Heavy Apps**: Ideal for our API-fetching bot

### **Compiler-Enforced Error Handling**
```rust
// Rust forces you to handle every possible error
let response = self.client
    .get("https://api.example.com/random.json")
    .timeout(Duration::from_secs(10))
    .send()
    .await?; // Must handle this error or code won't compile
```

**vs Other Languages:**
- **Python**: `try/except` - errors can be ignored
- **JavaScript**: `try/catch` - errors can be ignored
- **Go**: `if err != nil` - verbose but not enforced
- **Rust**: `?` operator - **compiler forces error handling**

### **Best-in-Class CLI Ecosystem**
```rust
// All the libraries we need are first-class
use clap::Parser;           // Argument parsing
use reqwest::Client;        // HTTP client
use serde::Deserialize;     // JSON handling
use teloxide::Bot;          // Telegram bot
use colored::*;             // Terminal colors
```

**Rust Libraries We Use:**
- **`clap`**: Best-in-class argument parsing with derive macros
- **`reqwest`**: HTTP client with async support and timeouts
- **`serde`**: JSON serialization/deserialization
- **`tokio`**: Async runtime for concurrent operations
- **`teloxide`**: Telegram bot framework
- **`colored`**: Terminal colors and styling
- **`dotenv`**: Environment variable loading

### **Single Binary Distribution**
```bash
# One file, no dependencies
./target/release/fact-cli --telegram --category all
```

**Benefits:**
- **Easy Deployment**: Just copy one file
- **No Dependencies**: No need to install Python, Node.js, etc.
- **Cross-Platform**: Works on Windows, macOS, Linux
- **Fast Startup**: No interpreter or runtime overhead

## 🔧 Development

### Running in Development
```bash
cargo run
cargo run -- --category coding --count 3
```

### Adding New Facts
Edit the `FactDatabase` struct in `src/main.rs` to add new facts to any category.

### Dependencies Explained
- **`clap`**: Command-line argument parsing with derive macros
- **`colored`**: Terminal colors and styling
- **`rand`**: Random number generation
- **`reqwest`**: HTTP client for API requests with async support
- **`serde`**: JSON serialization/deserialization
- **`tokio`**: Async runtime for concurrent operations
- **`teloxide`**: Telegram bot framework
- **`dotenv`**: Environment variable loading
- **`chrono`**: Date and time handling

## 🎯 Perfect Use Cases

This tool is ideal for:
- **Quick Learning Breaks**: Get a fact during coffee breaks
- **Terminal Fun**: Add some joy to your command line
- **System Design Study**: Learn about scaling and architecture
- **Programming History**: Discover the stories behind our tools
- **AI Exploration**: Stay updated with AI developments
- **Automated Learning**: Get facts delivered to Telegram every 6 hours

## 🚀 The Complete Picture

### **What We Built**
A production-ready CLI tool that:
1. **Fetches live data** from multiple internet APIs
2. **Falls back gracefully** to curated local content
3. **Runs reliably** 24/7 without crashes or memory leaks
4. **Handles errors** gracefully with compiler-enforced safety
5. **Distributes easily** as a single binary file
6. **Integrates with Telegram** for automated fact delivery

### **Why Rust Was Perfect**
- **Performance**: Fast execution for long-running processes
- **Safety**: Memory safety prevents crashes and leaks
- **Concurrency**: Excellent async support for API calls
- **Ecosystem**: Best-in-class CLI and web libraries
- **Distribution**: Single binary, no dependencies
- **Learning**: Teaches modern systems programming

### **Real-World Impact**
- **Educational**: Learn while having fun
- **Practical**: Actually useful for daily learning
- **Reliable**: Won't crash or consume resources
- **Maintainable**: Easy to extend and modify
- **Professional**: Production-quality code

## License

MIT License - feel free to use and modify!

---

**Happy Learning!** 🎉 This tool demonstrates how Rust can be used to build practical, educational, and fun applications that are both performant and reliable.
