use clap::Parser;
use rest_reminder::cli::Cli;
use std::env;
use rest_reminder::core::execute_command;
use rest_reminder::core::interactive::run_interactive_mode;

#[tokio::main]
async fn main() {
    let args: Vec<String> = env::args().collect();
    
    // If no command line arguments are provided (only program name), enter interactive mode
    if args.len() == 1 {
        run_interactive_mode().await;
    } else {
        // Parse and execute the command normally
        let cli = Cli::parse();
        execute_command(cli.cmd).await;
    }
}