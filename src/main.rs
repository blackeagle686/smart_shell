pub mod agent;
pub mod cli;
pub mod tools;

#[tokio::main]
async fn main() {
    cli::start_interactive().await;
}
