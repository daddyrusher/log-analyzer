# Log Analyzer

A command-line tool written in Rust for analyzing log files with support for pattern matching and time-based filtering. The tool can process logs in parallel, making it efficient for large log files.

## Features

- Fast parallel log processing using Rayon
- Pattern-based filtering
- Time range filtering
- Progress bar for visual feedback
- Log level statistics
- Multi-threaded processing

## Installation

Clone the repository and build using Cargo:

```bash
git clone https://github.com/yourusername/log-analyzer.git
cd log-analyzer
cargo build --release
```

The compiled binary will be available in `target/release/log-analyzer`.

## Usage

The tool supports various command-line arguments for flexible log analysis:

```bash
# Basic usage
log-analyzer -f path/to/logfile.log

# Filter logs containing specific pattern
log-analyzer -f logfile.log -p "ERROR"

# Filter logs within time range
log-analyzer -f logfile.log --from "2024-02-11 10:00:00" --to "2024-02-11 11:00:00"

# Combine pattern and time filtering
log-analyzer -f logfile.log -p "ERROR" --from "2024-02-11 10:00:00" --to "2024-02-11 11:00:00"

# Specify number of processing threads
log-analyzer -f logfile.log -t 8
```

### Command Line Arguments

- `-f, --file <PATH>` - Path to the log file
- `-p, --pattern <PATTERN>` - Pattern to search for in log messages
- `--from <DATETIME>` - Start time for filtering (format: YYYY-MM-DD HH:MM:SS)
- `--to <DATETIME>` - End time for filtering (format: YYYY-MM-DD HH:MM:SS)
- `-t, --threads <NUMBER>` - Number of processing threads (default: 4)

## Expected Log Format

The tool expects logs in the following format:
```
YYYY-MM-DD HH:MM:SS LEVEL Message
```

Example:
```
2024-02-11 10:00:00 INFO Starting application
2024-02-11 10:01:00 ERROR Database connection failed
```

## Development

### Project Structure

```
src/
├── lib.rs           # Library API
├── main.rs          # CLI entry point
├── models/          # Data structures
│   ├── mod.rs
│   ├── args.rs      # CLI arguments
│   └── log_entry.rs # Log entry parser
└── analyzer/        # Analysis logic
    └── mod.rs
```

### Running Tests

```bash
# Run all tests
cargo test

# Run tests with output
cargo test -- --nocapture
```

## Dependencies

- clap - Command line argument parsing
- rayon - Parallel processing
- chrono - Date and time handling
- anyhow - Error handling
- indicatif - Progress bars

## License

MIT License