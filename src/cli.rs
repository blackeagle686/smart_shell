use std::io::{self, Write};
use colored::*;
use crate::agent::{Agent, Llm};
use crate::tools::Tool;
use std::env;
use dotenvy::dotenv;

pub async fn start_interactive() {
    // Attempt to load .env file, ignore if not present
    let _ = dotenv();

    println!("{}", "===============================================".cyan().bold());
    println!("{}", " Welcome to Smart Shell AI Assistant".green().bold());
    println!("{}", " Type your request in plain English.".italic().cyan());
    println!("{}", " Type 'exit' or 'quit' to close.".italic().red());
    println!("{}", "===============================================\n".cyan().bold());

    let api_key = match env::var("API_KEY") {
        Ok(key) => key,
        Err(_) => {
            println!("{}", "⚠️  Warning: API_KEY environment variable not found.".yellow());
            println!("{}", "   Please set it in your environment or .env file.\n".yellow());
            String::new()
        }
    };

    let base_url = env::var("BASE_URL").unwrap_or_else(|_| "https://api.longcat.chat/openai/chat/completions".to_string());
    let model = env::var("MODEL").unwrap_or_else(|_| "LongCat-2.0-Preview".to_string());

    let llm = Llm::new(base_url, model, api_key);
    
    // Add default tools
    let tools = vec![
        Tool {
            name: "shell".to_string(),
            description: "Execute bash commands in the system shell".to_string(),
            is_human_in_loop: true, // Requires user confirmation by default
            command: "".to_string(),
        }
    ];

    let agent = Agent::new(llm, tools, 5);

    loop {
        print!("{}", "smart-shell> ".green().bold());
        io::stdout().flush().unwrap_or_default();

        let mut input = String::new();
        if io::stdin().read_line(&mut input).is_err() {
            println!("{}", "Error reading input".red());
            continue;
        }

        let input = input.trim();

        if input.eq_ignore_ascii_case("exit") || input.eq_ignore_ascii_case("quit") {
            println!("{}", "Goodbye!".yellow());
            break;
        }

        if input.is_empty() {
            continue;
        }

        println!("{} {}", "🧠 Thinking about:".cyan(), input.white());
        
        match agent.think(input).await {
            Ok(tasks) => {
                if tasks.is_empty() {
                    println!("{}", "No tasks generated for this request.".yellow());
                } else {
                    println!("{}", "📝 Tasks Planned:".magenta().bold());
                    for (i, task) in tasks.iter().enumerate() {
                        println!("  {}. {}", i + 1, task.title.white().bold());
                        println!("     {} {}", "Description:".bright_black(), task.description.bright_black());
                        println!("     {} {}", "Command:".yellow(), task.command.yellow().bold());
                        println!("     {} {} | {} {}", 
                            "Priority:".cyan(), task.priority,
                            "Human-in-loop:".cyan(), task.human_in_loop
                        );
                        println!();
                    }
                    
                    // Here you would prompt the user to execute them via agent.act()
                }
            }
            Err(e) => {
                println!("{} {}", "❌ Error:".red().bold(), e);
            }
        }
    }
}