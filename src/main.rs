use anyhow::Result;
use clap::Parser;
use log_analyzer::{Args, LogAnalyzer};

fn main() -> Result<()> {
    let args = Args::parse();
    let analyzer = LogAnalyzer::new(args);

    if !analyzer.args.path.exists() {
        anyhow::bail!("Specified path does not exist");
    }

    let entries = analyzer.analyze_file(&analyzer.args.path)?;
    analyzer.process_entries(entries)?;

    Ok(())
}