use std::path::PathBuf;
use std::fs::File;
use std::io::{BufRead, BufReader};
use rayon::prelude::*;
use anyhow::{Context, Result};
use indicatif::{ProgressBar, ProgressStyle};
use chrono::{NaiveDateTime, Utc};

use crate::models::args::Args;
use crate::models::log_entry::LogEntry;

pub struct LogAnalyzer {
    pub args: Args,
}

impl LogAnalyzer {
    pub fn new(args: Args) -> Self {
        Self { args }
    }

    pub fn analyze_file(&self, path: &PathBuf) -> Result<Vec<LogEntry>> {
        // Сначала подсчитаем количество строк
        let file = File::open(path)
            .with_context(|| format!("Failed to open file: {}", path.display()))?;
        let total_lines = BufReader::new(file).lines().count();

        // Теперь откроем файл заново для чтения
        let file = File::open(path)
            .with_context(|| format!("Failed to open file: {}", path.display()))?;
        let reader = BufReader::new(file);

        let pb = ProgressBar::new(total_lines as u64);
        pb.set_style(ProgressStyle::default_bar()
            .template("{spinner:.green} [{elapsed_precise}] [{bar:40.cyan/blue}] {pos}/{len} ({eta})")?);

        let entries: Vec<LogEntry> = reader
            .lines()
            .filter_map(|line| {
                pb.inc(1);
                line.ok().and_then(|l| LogEntry::parse(&l))
            })
            .collect();

        pb.finish_with_message("Analysis complete");
        Ok(entries)
    }

    pub fn process_entries(&self, entries: Vec<LogEntry>) -> Result<()> {
        let pool = rayon::ThreadPoolBuilder::new()
            .num_threads(self.args.threads)
            .build()?;

        pool.install(|| {
            let filtered_entries: Vec<_> = entries
                .par_iter()
                .filter(|entry| {
                    // Применяем фильтр по паттерну
                    if let Some(pattern) = &self.args.pattern {
                        if !entry.message.contains(pattern) {
                            return false;
                        }
                    }

                    // Проверяем временной диапазон
                    let in_time_range = match (&self.args.from, &self.args.to) {
                        (Some(from), Some(to)) => {
                            let from_time = NaiveDateTime::parse_from_str(from, "%Y-%m-%d %H:%M:%S")
                                .ok()
                                .and_then(|dt| Some(dt.and_local_timezone(Utc).unwrap()));

                            let to_time = NaiveDateTime::parse_from_str(to, "%Y-%m-%d %H:%M:%S")
                                .ok()
                                .and_then(|dt| Some(dt.and_local_timezone(Utc).unwrap()));

                            match (from_time, to_time) {
                                (Some(from_time), Some(to_time)) => {
                                    entry.timestamp >= from_time && entry.timestamp <= to_time
                                }
                                _ => false
                            }
                        }
                        (Some(from), None) => {
                            NaiveDateTime::parse_from_str(from, "%Y-%m-%d %H:%M:%S")
                                .ok()
                                .and_then(|dt| Some(dt.and_local_timezone(Utc).unwrap()))
                                .is_some_and(|from_time| entry.timestamp >= from_time)
                        }
                        (None, Some(to)) => {
                            NaiveDateTime::parse_from_str(to, "%Y-%m-%d %H:%M:%S")
                                .ok()
                                .and_then(|dt| Some(dt.and_local_timezone(Utc).unwrap()))
                                .is_some_and(|to_time| entry.timestamp <= to_time)
                        }
                        (None, None) => true,
                    };

                    in_time_range
                })
                .collect();

            self.print_statistics(&filtered_entries);
            Ok(())
        })
    }

    pub fn validate_path(&self) -> Result<()> {
        if !self.args.path.exists() {
            anyhow::bail!("Specified path does not exist");
        }
        Ok(())
    }

    fn print_statistics(&self, entries: &[&LogEntry]) {
        println!("\nAnalysis Results:");
        println!("Total entries processed: {}", entries.len());

        let mut level_counts: std::collections::HashMap<String, usize> = std::collections::HashMap::new();
        for entry in entries {
            *level_counts.entry(entry.level.clone()).or_insert(0) += 1;
        }

        println!("\nLog Level Distribution:");
        for (level, count) in level_counts {
            println!("{}: {}", level, count);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs::write;
    use tempfile::NamedTempFile;

    fn create_test_log_file(content: &str) -> NamedTempFile {
        let file = NamedTempFile::new().unwrap();
        write(file.path(), content).unwrap();
        file
    }

    #[test]
    fn test_analyze_file() {
        let log_content = "2025-02-11 10:00:00 INFO Test message\n2025-02-11 10:01:00 ERROR Test error";
        let log_file = create_test_log_file(log_content);

        let args = Args {
            path: log_file.path().to_path_buf(),
            pattern: None,
            from: None,
            to: None,
            threads: 1,
        };

        let analyzer = LogAnalyzer::new(args);
        let entries = analyzer.analyze_file(&log_file.path().to_path_buf()).unwrap();

        assert_eq!(entries.len(), 2);
        assert_eq!(entries[0].level, "INFO");
        assert_eq!(entries[1].level, "ERROR");
    }
}