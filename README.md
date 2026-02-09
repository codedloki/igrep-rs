## 🔍 igrep

A lightweight, high-performance command-line utility for pattern searching within files, featuring real-time terminal highlighting. Built with safety and speed in mind using the Rust standard library.

### 🚀 Overview

igrep (Instant Grep) simplifies file searching by providing a clean, colorized output that makes finding specific data in large text files effortless. Unlike standard tools, igrep focuses on visual clarity through ANSI-coded highlighting.
✨ Features

    ⚡ Speed: Leverages Rust's zero-cost abstractions for rapid file processing.

    🎨 Visual Highlighting: Automatically bolds and colors matches in Green for easy scanning.

    🛡️ Robust Error Handling: Graceful exits with user-friendly instructions on incorrect usage.

    📦 Zero Dependencies: No external crates required; keeps the binary footprint tiny.

🛠️ Installation

Ensure you have the Rust toolchain installed. If not, get it at rustup.rs.
Bash

# Clone the repository
git clone https://github.com/your-username/igrep.git

# Navigate to the directory
cd igrep

# Build the optimized release version
cargo build --release

The executable will be available at ./target/release/igrep.
📖 Usage

Invoke igrep from your terminal by providing a file path and the pattern you wish to locate.
Bash

cargo run -- <file_path> <pattern>

Example
Bash

cargo run -- src/main.rs println

Arguments
Argument	Description
file_path	The relative or absolute path to the target text file.
pattern	The string or word you are searching for.
💻 Internal Logic

The tool operates via a streamlined pipeline:

    Argument Parsing: Validates std::env::args.

    File Streaming: Loads file content via std::fs::read_to_string.

    Buffer Iteration: Uses .lines() to process data without loading the entire file into memory multiple times.

    ANSI Injection: Maps search hits to the string \x1b[32;1m{pattern}\x1b[0m.

🗺️ Roadmap

    [ ] Case-Insensitive Search: Toggle matching regardless of casing.

    [ ] Line Numbering: Prefix matches with their corresponding line number.

    [ ] Regex Support: Enable advanced pattern matching.

    [ ] Recursive Search: Search through entire directories.

📄 License

Distributed under the MIT License. See LICENSE for more information.
