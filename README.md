igrep

igrep is a lightweight, CLI-based pattern searcher written in Rust. It mimics the basic functionality of grep but adds immediate visual feedback by highlighting matching words directly in your terminal.
Features

    Instant Highlighting: Automatically wraps matching patterns in bold green for high visibility.

    Error Handling: Intelligent argument checking that provides a helpful usage guide if parameters are missing.

    Memory Efficient: Processes files line-by-line using Rust's lines() iterator.

    Zero Dependencies: Built entirely with the Rust standard library (std).

Installation

To use igrep, you must have Rust and Cargo installed on your machine.

    Clone the repository:
    Bash

    git clone https://github.com/your-username/igrep.git
    cd igrep

    Build the project:
    Bash

    cargo build --release

Usage

Run the program by passing the target file path and the word you want to search for:
Bash

cargo run -- <file_path> <word>

Example

If you have a file named notes.txt and want to find the word "Rust":
Bash

cargo run -- notes.txt Rust

Technical Overview

The tool follows a simple but effective logic flow:

    Argument Collection: Uses env::args() to capture user input.

    Validation: Checks if at least two arguments (path and pattern) are provided.

    File I/O: Reads the entire file content into a string buffer.

    Pattern Matching: Iterates through each line and uses .contains() to find matches.

    ANSI Styling: Uses the escape code \x1b[32;1m to inject color before printing the result.

Roadmap

    [ ] Add case-insensitive search support.

    [ ] Add line numbering for matches.

    [ ] Support for multiple files.

    [ ] Regular Expression (Regex) support.

License

This project is licensed under the MIT License.
