use anyhow::Result;
use clap::Parser;
use git2::Repository;

use git_stats::{
    tui,
    users::{self, update_user_stats},
};

#[derive(Parser)]
#[command(version, about, long_about = None)]
pub struct Args {
    /// Path to the git repository
    /// If not specified, the current directory will be used
    #[arg(short, long, default_value = ".")]
    pub repo: String,
    /// Branch to analyze
    /// If not specified, the current branch will be used
    #[arg(short, long, default_value = "HEAD")]
    pub branch: String,
    /// Use TUI interface
    #[arg(short, long)]
    pub tui: bool,
}

#[tokio::main]
async fn main() -> Result<()> {
    if std::env::var("RUST_BACKTRACE").is_err() {
        unsafe {
            std::env::set_var("RUST_BACKTRACE", "1");
        }
    }

    let options = Args::parse();
    // Open the git repository
    let path = options.repo;
    let branch = options.branch;

    let repo = Repository::open(path)?;
    let mut users = users::get_users(&repo)?;
    update_user_stats(&mut users, &repo, &branch)?;

    // Sort the users by total commits
    users.sort_by(|a, b| b.stats.total_commits.cmp(&a.stats.total_commits));

    // Filter out user with no stats or less then 10 commits
    users.retain(|u| !u.has_not_stats() && u.stats.total_commits > 10);

    if options.tui {
        // Launch TUI
        tui::run_tui(users)?;
    } else {
        // Print the users in CLI mode
        for user in users.iter() {
            println!("User: {} <{}>", user.name, user.email);
            println!("Total commits: {}", user.stats.total_commits);
            println!(
                "Average commits per day: {}",
                user.stats.average_commits_per_day
            );
            println!(
                "Average commits per month: {}",
                user.stats.average_commits_per_month
            );
            println!(
                "Average commits per year: {}",
                user.stats.average_commits_per_year
            );
            println!("Total lines added: {}", user.stats.total_lines_added);
            println!("Total lines removed: {}", user.stats.total_lines_removed);
            println!(
                "Average lines added per day: {}",
                user.stats.average_lines_added_per_day
            );
            println!(
                "Average lines removed per day: {}",
                user.stats.average_lines_removed_per_day
            );
            println!(
                "Average lines added per month: {}",
                user.stats.average_lines_added_per_month
            );
            println!(
                "Average lines removed per month: {}",
                user.stats.average_lines_removed_per_month
            );
            println!(
                "Average lines added per year: {}",
                user.stats.average_lines_added_per_year
            );
            println!("-----------------------------------");
        }
    }

    Ok(())
}
