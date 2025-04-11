# git-stats

This came as an idea from @arturleao. An easy way to get the stats of a git repository.

A Rust command-line tool to analyze git repositories and generate detailed contributor statistics.

> ⚠️ **Note:** This project is still a work in progress.

## Features

- User contribution analysis
- Commit frequency metrics
- Code change statistics (lines added/removed)
- Per-day, per-month, and per-year aggregated statistics
- Filter capabilities for meaningful data extraction

## Installation

### Prerequisites

- Rust and Cargo installed on your system
- Git 2.0 or higher

### Building from source

```bash
# Clone the repository
git clone https://github.com/yourusername/git-stats.git
cd git-stats

# Build the project
cargo build --release

# The binary will be available at ./target/release/git-stats
```

## Usage

```bash
# Analyze the current directory's git repository
git-stats

# Analyze a specific repository
git-stats --path /path/to/repository

# Analyze a specific branch
git-stats --branch main

# Combine options
git-stats --path /path/to/repository --branch develop
```

### Command-line Options

- `-p, --path <PATH>` - Path to the git repository (defaults to current directory)
- `-b, --branch <BRANCH>` - Branch to analyze (defaults to HEAD)
- `-h, --help` - Display help information
- `-V, --version` - Display version information

## Example Output

```
User: John Doe <john@example.com>
Total commits: 183
Average commits per day: 2.5
Average commits per month: 15.3
Average commits per year: 91.5
Total lines added: 12450
Total lines removed: 8320
Average lines added per day: 170.5
Average lines removed per day: 113.9
-----------------------------------
```

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.
