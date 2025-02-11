use std::path::PathBuf;
use clap::Parser;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct Args {
    /// Path to log file or directory
    #[arg(short = 'f', long = "file")]
    pub path: PathBuf,

    /// Pattern to search for
    #[arg(short = 'p', long = "pattern")]
    pub pattern: Option<String>,

    /// Start time for filtering (format: YYYY-MM-DD HH:MM:SS)
    #[arg(long)]
    pub from: Option<String>,

    /// End time for filtering (format: YYYY-MM-DD HH:MM:SS)
    #[arg(long)]
    pub to: Option<String>,

    /// Number of threads for parallel processing
    #[arg(short, long, default_value_t = 4)]
    pub threads: usize,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_args_parsing() {
        let args = Args::parse_from([
            "test",
            "-p", "test.log",
            "--pattern", "ERROR",
            "--from", "2025-02-11 00:00:00",
            "--threads", "2",
        ].iter());

        assert_eq!(args.path, PathBuf::from("test.log"));
        assert_eq!(args.pattern, Some("ERROR".to_string()));
        assert_eq!(args.from, Some("2025-02-11 00:00:00".to_string()));
        assert_eq!(args.threads, 2);
    }
}