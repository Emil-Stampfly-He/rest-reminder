use clap::Parser;
use rustyline::DefaultEditor;
use crate::cli::{Cli, Command};
use colored::*;
use crate::core::execute_command;

pub async fn run_interactive_mode() {
    println!("{}", "Welcome to Rest Reminder Interactive Mode".bright_cyan().bold());
    println!("{}", "═══════════════════════════════════════════════".bright_cyan());
    println!("{}", "Available commands:".bright_green().bold());
    println!("  {}              - {}", "rest".yellow().bold(), "Start monitoring and remind you to rest".white());
    println!("  {}             - {}", "count".yellow().bold(), "Count work time between two days".white());
    println!("  {}  - {}", "count-single-day".yellow().bold(), "Count work time for a specific day".white());
    println!("  {}     - {}", "count-precise".yellow().bold(), "Count work time between precise timestamps".white());
    println!("  {}              - {}", "plot".yellow().bold(), "Generate work time trend plot".white());
    println!("  {}               - {}", "gen".yellow().bold(), "(FOR DEV USE ONLY) Generate plugin template".white());
    println!("  {}               - {}", "web".yellow().bold(), "Start web mode".white());
    println!("  {}              - {}", "help".yellow().bold(), "Show this help message".white());
    println!("  {}       - {}", "exit / quit".yellow().bold(), "Exit the program".white());
    println!("{}", "═══════════════════════════════════════════════".bright_cyan());
    println!("{}: {}", "Type a command and press Enter. Example", "rest -t 3600 -a Cursor -l ~/Desktop/focus_log.txt".green());
    println!();

    let mut rl = DefaultEditor::new().expect("Failed to initialize readline");

    // Try to load history from file
    let _ = rl.load_history("rest_reminder_history.txt");

    loop {
        let readline = rl.readline("RestReminder> ");
        match readline {
            Ok(line) => {
                let line = line.trim();

                if line.is_empty() {
                    continue;
                }

                // Add command to history
                let _ = rl.add_history_entry(line);

                // Handle special commands
                match line {
                    "exit" | "quit" | "q" | "ex" => {
                        println!("{}", "Goodbye! Thank you for using Rest Reminder!".blue().bold());
                        break;
                    }
                    "help" | "h" => {
                        show_help();
                        continue;
                    }
                    _ => {}
                }

                // Parse and execute the command
                if let Some(command) = parse_interactive_command(line) {
                    execute_command(command).await;
                } else {
                    println!("{}", "Invalid command. Type 'help' for available commands.".red().bold());
                }
            }
            Err(rustyline::error::ReadlineError::Interrupted) => {
                println!("{}", "Goodbye! Thank you for using Rest Reminder!".blue().bold());
                break;
            }
            Err(rustyline::error::ReadlineError::Eof) => {
                println!("{}", "Goodbye! Thank you for using Rest Reminder!".blue().bold());
                break;
            }
            Err(err) => {
                println!("Error reading input: {}", err);
                break;
            }
        }
    }

    // Save command history
    let _ = rl.save_history("rest_reminder_history.txt");
}

fn parse_interactive_command(input: &str) -> Option<Command> {
    // Prepend "rest-reminder" to make it parseable by clap
    let full_args = format!("rest-reminder {}", input);
    let args: Vec<&str> = full_args.split_whitespace().collect();

    // Try to parse the command
    match Cli::try_parse_from(args) {
        Ok(cli) => Some(cli.cmd),
        Err(e) => {
            println!("Error parsing command: {}", e);
            None
        }
    }
}

fn show_help() {
    println!();
    println!("{}", "Rest Reminder Commands:".bright_cyan().bold());
    println!("{}", "─────────────────────────────".bright_cyan());
    println!();
    
    println!("{}", "MONITORING:".bright_green().bold());
    println!("  {}", "rest [OPTIONS]".yellow().bold());
    println!("    {}     {}", "-l, --log-to <PATH>".blue(), "Log file location".white());
    println!("    {}    {}", "-t, --time <SECONDS>".blue(), "Work time before reminder (default: 3600)".white());
    println!("    {}      {}", "-a, --app <APP>...".blue(), "Applications to monitor".white());
    println!("    {}: {}", "Example".bright_magenta(), "rest -t 1800 -a Cursor Code".green());
    println!();
    
    println!("{}", "STATISTICS:".bright_green().bold());
    println!("  {}", "count [OPTIONS]".yellow().bold());
    println!("    {}  {}", "-l, --log-location <PATH>".blue(), "Log file path".white());
    println!("    {}         {}", "-s, --start <DATE>".blue(), "Start date (YYYY-MM-DD)".white());
    println!("    {}           {}", "-e, --end <DATE>".blue(), "End date (YYYY-MM-DD)".white());
    println!("    {}: {}", "Example".bright_magenta(), "count -s 2024-01-01 -e 2024-01-31".green());
    println!();
    
    println!("  {}", "count-single-day [OPTIONS]".yellow().bold());
    println!("    {}  {}", "-l, --log-location <PATH>".blue(), "Log file path".white());
    println!("    {}           {}", "-d, --day <DATE>".blue(), "Date (YYYY-MM-DD)".white());
    println!("    {}: {}", "Example".bright_magenta(), "count-single-day -d 2024-01-15".green());
    println!();
    
    println!("  {}", "count-precise [OPTIONS]".yellow().bold());
    println!("    {}  {}", "-l, --log-location <PATH>".blue(), "Log file path".white());
    println!("    {}     {}", "-s, --start <DATETIME>".blue(), "Start time (YYYY-MM-DD HH:MM:SS)".white());
    println!("    {}       {}", "-e, --end <DATETIME>".blue(), "End time (YYYY-MM-DD HH:MM:SS)".white());
    println!("    {}: {}", "Example".bright_magenta(), "count-precise -s \"2024-01-15 09:00:00\" -e \"2024-01-15 17:00:00\"".green());
    println!();
    
    println!("{}", "VISUALIZATION:".bright_green().bold());
    println!("  {}", "plot [OPTIONS]".yellow().bold());
    println!("    {}   {}", "-l, --log-location <PATH>".blue(), "Log file path".white());
    println!("    {}  {}", "-p, --plot-location <PATH>".blue(), "Output plot file path".white());
    println!("    {}          {}", "-s, --start <DATE>".blue(), "Start date (YYYY-MM-DD)".white());
    println!("    {}            {}", "-e, --end <DATE>".blue(), "End date (YYYY-MM-DD)".white());
    println!("    {}: {}", "Example".bright_magenta(), "plot -s 2024-01-01 -e 2024-01-31 -p ~/work_trend.png".green());
    println!();

    println!("{}", "TEMPLATE GENERATOR:".bright_green().bold());
    println!("  {}", "gen [OPTIONS]".yellow().bold());
    println!("    {}   {}", "-n, --name <FILENAME>".blue(), "Template file name".white());
    println!();

    println!("{}", "WEB MODE STARTER:".bright_green().bold());
    println!("  {}", "web".yellow().bold());
    println!();
    
    println!("{}", "SYSTEM:".bright_green().bold());
    println!("  {}                {}", "help, h".yellow().bold(), "Show this help message".white());
    println!("  {}      {}", "exit, quit, q, ex".yellow().bold(), "Exit interactive mode".white());
    println!();
}