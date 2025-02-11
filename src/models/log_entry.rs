use chrono::{DateTime, NaiveDateTime, Utc};

#[derive(Debug)]
pub struct LogEntry {
    pub timestamp: DateTime<Utc>,
    pub level: String,
    pub message: String,
}

impl LogEntry {
    pub fn parse(line: &str) -> Option<Self> {
        let parts: Vec<&str> = line.splitn(3, ' ').collect();
        if parts.len() < 3 {
            return None;
        }

        let timestamp = NaiveDateTime::parse_from_str(parts[0], "%Y-%m-%d %H:%M:%S")
            .ok()?
            .and_local_timezone(Utc)
            .unwrap();

        Some(LogEntry {
            timestamp,
            level: parts[1].to_string(),
            message: parts[2].to_string(),
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_valid_log_entry() {
        let line = "2025-02-11 10:15:30 ERROR Failed to connect to database";
        let entry = LogEntry::parse(line).unwrap();

        assert_eq!(entry.level, "ERROR");
        assert_eq!(entry.message, "Failed to connect to database");
    }

    #[test]
    fn test_parse_invalid_log_entry() {
        let line = "invalid log entry format";
        assert!(LogEntry::parse(line).is_none());
    }

    #[test]
    fn test_parse_incomplete_log_entry() {
        let line = "2025-02-11 10:15:30";
        assert!(LogEntry::parse(line).is_none());
    }
}