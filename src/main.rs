use clap::Parser;
use colored::*;
use rand::Rng;
use reqwest::Client;
use serde::{Deserialize, Serialize};
use std::io::{self, Write};
use teloxide::{prelude::*, types::ParseMode};
use tokio::time::{sleep, Duration};
use dotenv::dotenv;
use std::env;

/// A CLI tool that fetches random coding/AI/scaling facts from APIs and can send them to Telegram
#[derive(Parser)]
#[command(name = "fact-cli")]
#[command(version)]
#[command(about = "A fun CLI tool that fetches random coding, AI, and scaling facts from APIs!")]
#[command(long_about = "Get inspired with random facts about programming, artificial intelligence, and system scaling. Can also send facts to Telegram every 6 hours!")]
struct Cli {
    /// Number of facts to display (default: 1)
    #[arg(short, long, default_value = "1")]
    count: u32,
    
    /// Category of facts to display
    #[arg(short = 't', long, value_enum, default_value = "all")]
    category: Category,
    
    /// Display facts continuously (press Ctrl+C to stop)
    #[arg(short, long)]
    watch: bool,
    
    /// Start Telegram bot mode (sends facts every 6 hours)
    #[arg(short = 'b', long)]
    telegram: bool,
}

#[derive(clap::ValueEnum, Clone)]
enum Category {
    All,
    Coding,
    Ai,
    Scaling,
    Tech,
    Programming,
}

// API Response structures
#[derive(Deserialize)]
struct UselessFact {
    text: String,
}

#[derive(Deserialize)]
struct ChuckNorrisFact {
    value: String,
}

#[derive(Deserialize)]
struct CatFact {
    text: String,
}

#[derive(Deserialize)]
struct TechFact {
    fact: String,
}

#[derive(Deserialize)]
struct ProgrammingFact {
    fact: String,
}

// Configuration for the bot
#[derive(Serialize, Deserialize)]
struct Config {
    telegram_bot_token: String,
    telegram_chat_id: String,
}

impl Config {
    fn from_env() -> Result<Self, String> {
        let telegram_bot_token = env::var("TELEGRAM_BOT_TOKEN")
            .map_err(|_| "TELEGRAM_BOT_TOKEN environment variable not set")?;
        let telegram_chat_id = env::var("TELEGRAM_CHAT_ID")
            .map_err(|_| "TELEGRAM_CHAT_ID environment variable not set")?;
        
        Ok(Config {
            telegram_bot_token,
            telegram_chat_id,
        })
    }
}

// Fact fetcher that uses multiple APIs
struct FactFetcher {
    client: Client,
}

impl FactFetcher {
    fn new() -> Self {
        Self {
            client: Client::new(),
        }
    }

    async fn fetch_useless_fact(&self) -> Result<String, Box<dyn std::error::Error>> {
        let response = self.client
            .get("https://uselessfacts.jsph.pl/random.json?language=en")
            .timeout(Duration::from_secs(10))
            .send()
            .await?;
        
        if !response.status().is_success() {
            return Err("API returned non-success status".into());
        }
        
        let fact: UselessFact = response.json().await?;
        Ok(fact.text)
    }

    async fn fetch_chuck_norris_fact(&self) -> Result<String, Box<dyn std::error::Error>> {
        let response = self.client
            .get("https://api.chucknorris.io/jokes/random")
            .timeout(Duration::from_secs(10))
            .send()
            .await?;
        
        if !response.status().is_success() {
            return Err("API returned non-success status".into());
        }
        
        let fact: ChuckNorrisFact = response.json().await?;
        Ok(fact.value)
    }

    async fn fetch_cat_fact(&self) -> Result<String, Box<dyn std::error::Error>> {
        let response = self.client
            .get("https://cat-fact.herokuapp.com/facts/random")
            .timeout(Duration::from_secs(10))
            .send()
            .await?;
        
        if !response.status().is_success() {
            return Err("API returned non-success status".into());
        }
        
        let fact: CatFact = response.json().await?;
        Ok(fact.text)
    }

    async fn fetch_tech_fact(&self) -> Result<String, Box<dyn std::error::Error>> {
        // Using a tech facts API (this is a mock - you'd need to find a real one)
        // For now, we'll use some hardcoded tech facts as fallback
        let tech_facts = vec![
            "The first computer bug was an actual bug! Grace Hopper found a moth stuck in a Harvard Mark II computer in 1947.",
            "The term 'debugging' comes from this incident - they literally had to 'debug' the computer by removing the moth.",
            "The first programming language was FORTRAN, created by IBM in 1957. It's still used today in scientific computing!",
            "Linus Torvalds created Linux as a hobby project while he was a student at the University of Helsinki in 1991.",
            "The first computer virus was created in 1983 by Fred Cohen and was called 'Elk Cloner'.",
            "The word 'algorithm' comes from the name of a 9th-century Persian mathematician, Al-Khwarizmi.",
            "The first website ever created is still online: http://info.cern.ch/hypertext/WWW/TheProject.html",
            "JavaScript was created in just 10 days by Brendan Eich in 1995. He was originally hired to work on Scheme!",
            "The first computer programmer was Ada Lovelace, who wrote algorithms for Charles Babbage's Analytical Engine in the 1840s.",
            "Git was created by Linus Torvalds in 2005 because he was frustrated with existing version control systems.",
        ];
        
        let mut rng = rand::thread_rng();
        Ok(tech_facts[rng.gen_range(0..tech_facts.len())].to_string())
    }

    async fn fetch_programming_fact(&self) -> Result<String, Box<dyn std::error::Error>> {
        // Using a programming facts API (this is a mock - you'd need to find a real one)
        let programming_facts = vec![
            "The 'Hello, World!' program was first used in a 1972 book by Brian Kernighan and Dennis Ritchie.",
            "The first computer mouse was made of wood and had only one button. It was invented by Douglas Engelbart in 1964.",
            "The term 'software' was first used by John Tukey in 1958, though the concept existed before that.",
            "The first computer game was 'Spacewar!' created in 1962 by Steve Russell at MIT.",
            "The first email was sent by Ray Tomlinson in 1971. He can't remember what it said!",
            "The first computer was the size of a room and had less processing power than a modern calculator.",
            "The first computer programmer was a woman: Ada Lovelace, who worked on Charles Babbage's Analytical Engine.",
            "The first computer virus was created in 1983 and was called 'Elk Cloner'.",
            "The first computer bug was an actual bug - a moth found in a Harvard Mark II computer in 1947.",
            "The first computer mouse was made of wood and had only one button.",
        ];
        
        let mut rng = rand::thread_rng();
        Ok(programming_facts[rng.gen_range(0..programming_facts.len())].to_string())
    }

    async fn fetch_random_fact(&self, category: &Category) -> Result<String, Box<dyn std::error::Error>> {
        let mut rng = rand::thread_rng();
        let api_choice = rng.gen_range(0..4);

        match category {
            Category::All => {
                // Try APIs in order, fallback to local facts if all fail
                if let Ok(fact) = self.fetch_useless_fact().await {
                    return Ok(fact);
                }
                if let Ok(fact) = self.fetch_chuck_norris_fact().await {
                    return Ok(fact);
                }
                if let Ok(fact) = self.fetch_cat_fact().await {
                    return Ok(fact);
                }
                
                // If all APIs fail, use local tech facts as fallback
                self.fetch_tech_fact().await
            }
            Category::Coding | Category::Programming => {
                self.fetch_programming_fact().await
            }
            Category::Tech => {
                self.fetch_tech_fact().await
            }
            Category::Ai => {
                // AI facts - using hardcoded for now
                let ai_facts = vec![
                    "The term 'Artificial Intelligence' was coined by John McCarthy in 1956 at the Dartmouth Conference.",
                    "The first AI program was written in 1951 by Christopher Strachey - it played checkers!",
                    "Machine learning algorithms can now detect cancer in medical images with higher accuracy than human radiologists.",
                    "The first chatbot, ELIZA, was created in 1966 by Joseph Weizenbaum at MIT. It simulated a psychotherapist.",
                    "Deep Blue, IBM's chess computer, defeated world champion Garry Kasparov in 1997 - a historic moment for AI.",
                    "The concept of neural networks dates back to 1943, when Warren McCulloch and Walter Pitts created the first mathematical model.",
                    "GPT-3 has 175 billion parameters, but the human brain has approximately 86 billion neurons with 100 trillion connections.",
                    "The first self-driving car was developed by Carnegie Mellon University in 1984 - it could reach speeds of 20 mph.",
                    "AI can now generate code, but it still struggles with complex reasoning and understanding context like humans do.",
                    "The Turing Test, proposed by Alan Turing in 1950, is still considered a benchmark for AI intelligence.",
                ];
                let mut rng = rand::thread_rng();
                Ok(ai_facts[rng.gen_range(0..ai_facts.len())].to_string())
            }
            Category::Scaling => {
                // Scaling facts - using hardcoded for now
                let scaling_facts = vec![
                    "Google processes over 8.5 billion searches per day - that's about 99,000 searches per second!",
                    "Facebook (Meta) serves over 2.9 billion monthly active users with their distributed systems architecture.",
                    "Amazon's AWS handles more than 1 million requests per second during peak times.",
                    "The CAP theorem states that a distributed system can only guarantee two of three properties: Consistency, Availability, and Partition tolerance.",
                    "Microservices architecture allows companies like Netflix to deploy code hundreds of times per day.",
                    "Load balancing can distribute traffic across multiple servers, but it requires careful session management.",
                    "Database sharding splits large databases into smaller, more manageable pieces across multiple servers.",
                    "CDNs (Content Delivery Networks) can reduce page load times by 50-70% by serving content from locations closer to users.",
                    "Horizontal scaling (adding more servers) is often more cost-effective than vertical scaling (upgrading hardware).",
                    "Caching can improve application performance by 10-100x by storing frequently accessed data in memory.",
                ];
                let mut rng = rand::thread_rng();
                Ok(scaling_facts[rng.gen_range(0..scaling_facts.len())].to_string())
            }
        }
    }
}

fn get_category_emoji(category: &Category) -> &'static str {
    match category {
        Category::All => "🌟",
        Category::Coding => "💻",
        Category::Programming => "💻",
        Category::Ai => "🤖",
        Category::Scaling => "📈",
        Category::Tech => "⚡",
    }
}

fn print_fact(fact: &str, category: &Category, index: u32) {
    let emoji = get_category_emoji(category);
    let category_name = match category {
        Category::All => "Random",
        Category::Coding => "Coding",
        Category::Programming => "Programming",
        Category::Ai => "AI",
        Category::Scaling => "Scaling",
        Category::Tech => "Tech",
    };

    println!();
    println!("{} {} {} {}", 
        "┌─".bright_blue(), 
        emoji, 
        category_name.bright_cyan().bold(), 
        "Fact".bright_cyan().bold()
    );
    
    if index > 1 {
        println!("{} {}", "│".bright_blue(), format!("#{index}").bright_yellow());
    }
    
    println!("{}", "├─".bright_blue());
    println!("{} {}", "│".bright_blue(), fact.bright_white());
    println!("{}", "└─".bright_blue());
    println!();
}

async fn send_telegram_fact(bot: &Bot, chat_id: &str, fact: &str, category: &Category) -> Result<(), Box<dyn std::error::Error>> {
    let emoji = get_category_emoji(category);
    let category_name = match category {
        Category::All => "Random",
        Category::Coding => "Coding",
        Category::Programming => "Programming",
        Category::Ai => "AI",
        Category::Scaling => "Scaling",
        Category::Tech => "Tech",
    };

    let message = format!("{} {} {} Fact\n\n{}", emoji, category_name, "Random", fact);
    
    // Send as plain text instead of MarkdownV2 to avoid parsing issues
    bot.send_message(ChatId(chat_id.parse::<i64>()?), message)
        .await?;
    
    Ok(())
}

async fn run_telegram_bot(config: Config, category: Category) -> Result<(), Box<dyn std::error::Error>> {
    let bot = Bot::new(&config.telegram_bot_token);
    let fetcher = FactFetcher::new();
    
    println!("{}", "🤖 Telegram bot started! Sending facts every 6 hours...".bright_green().bold());
    println!("{}", "Press Ctrl+C to stop.".bright_yellow());
    
    loop {
        match fetcher.fetch_random_fact(&category).await {
            Ok(fact) => {
                println!("{}", "📡 Fetched new fact from API!".bright_blue());
                if let Err(e) = send_telegram_fact(&bot, &config.telegram_chat_id, &fact, &category).await {
                    eprintln!("❌ Error sending to Telegram: {}", e);
                } else {
                    println!("✅ Fact sent to Telegram successfully!");
                }
            }
            Err(e) => {
                eprintln!("❌ Error fetching fact: {}", e);
            }
        }
        
        println!("{}", "⏰ Waiting 6 hours for next fact...".bright_cyan());
        sleep(Duration::from_secs(6 * 3600)).await; // 6 hours
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Load environment variables
    dotenv().ok();
    
    let cli = Cli::parse();
    let fetcher = FactFetcher::new();

    if cli.telegram {
        let config = Config::from_env()
            .map_err(|e| format!("Configuration error: {}", e))?;
        
        run_telegram_bot(config, cli.category).await?;
        return Ok(());
    }

    if cli.watch {
        println!("{}", "🔄 Watch mode enabled! Press Ctrl+C to stop.".bright_green().bold());
        println!("{}", "Fetching random facts every 3 seconds...\n".bright_yellow());
        
        loop {
            for i in 1..=cli.count {
                match fetcher.fetch_random_fact(&cli.category).await {
                    Ok(fact) => print_fact(&fact, &cli.category, i),
                    Err(e) => {
                        eprintln!("❌ Error fetching fact: {}", e);
                        break;
                    }
                }
            }
            
            print!("{}", "Press Enter for next batch or Ctrl+C to exit... ".bright_cyan());
            io::stdout().flush().unwrap();
            
            let mut input = String::new();
            io::stdin().read_line(&mut input).unwrap();
        }
    } else {
        for i in 1..=cli.count {
            match fetcher.fetch_random_fact(&cli.category).await {
                Ok(fact) => print_fact(&fact, &cli.category, i),
                Err(e) => {
                    eprintln!("❌ Error fetching fact: {}", e);
                    break;
                }
            }
        }
    }

    Ok(())
}