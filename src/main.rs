use anyhow::Result;
use clap::Parser;
use log_analyzer::{Args, LogAnalyzer};

fn main() -> Result<()> {
    let args = Args::parse();
    let analyzer = LogAnalyzer::new(args);

    analyzer.validate_path()?;

    let entries = analyzer.analyze_file(&analyzer.args.path)?;
    analyzer.process_entries(entries)?;

    Ok(())
}