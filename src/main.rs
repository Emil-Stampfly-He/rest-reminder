use clap::Parser;
use rest_reminder::core::{run_rest_reminder, Cli, Command};

fn main() {
    let cli = Cli::parse();

    match cli.cmd {
        Command::CountTime => {
            // TODO statistics();
        }
        Command::Rest { log_to, time, app } => {
            run_rest_reminder(log_to, time, app);
        }
    }
}




