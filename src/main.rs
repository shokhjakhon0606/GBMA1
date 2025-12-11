use clap::{Parser, Subcommand};

/// Track your study time from the command line.
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand, Debug)]
enum Commands {
    /// Log a study session: clistudy log <minutes> <topic>
    Log {
        /// How many minutes did you study?
        minutes: i64,
        /// What did you work on? e.g. "Rust exam prep"
        topic: String,
    },

    /// Show today's study summary
    Today,

    /// Show last 7 days summary
    Week,
}

fn main() {
    let cli = Cli::parse();

    match cli.command {
        Commands::Log { minutes, topic } => {
            if minutes <= 0 {
                eprintln!("Minutes must be positive.");
                std::process::exit(1);
            }

            if let Err(e) = clistudy::add_session(minutes, topic.clone()) {
                eprintln!("Error saving session: {e}");
                std::process::exit(1);
            } else {
                println!("Logged {minutes} minutes for '{topic}'.");
            }
        }

        Commands::Today => {
            match clistudy::summary_today() {
                Ok(summary) => {
                    if summary.is_empty() {
                        println!("No sessions logged for today yet.");
                    } else {
                        println!("Today's study summary:");
                        print_summary(&summary);
                    }
                }
                Err(e) => {
                    eprintln!("Error reading data: {e}");
                    std::process::exit(1);
                }
            }
        }

        Commands::Week => {
            match clistudy::summary_week() {
                Ok(summary) => {
                    if summary.is_empty() {
                        println!("No sessions logged in the last 7 days.");
                    } else {
                        println!("Last 7 days summary:");
                        print_summary(&summary);
                    }
                }
                Err(e) => {
                    eprintln!("Error reading data: {e}");
                    std::process::exit(1);
                }
            }
        }
    }
}

fn print_summary(summary: &std::collections::HashMap<String, i64>) {
    let mut entries: Vec<_> = summary.iter().collect();
    // sort by minutes desc
    entries.sort_by(|a, b| b.1.cmp(a.1));

    let total: i64 = entries.iter().map(|(_, m)| *m).sum();

    for (topic, minutes) in entries {
        println!("- {topic}: {minutes} min");
    }
    println!("-------------------------");
    println!("Total: {total} min");
}