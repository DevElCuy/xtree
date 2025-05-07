# xtree

A lightweight directory tree generator written in Rust, inspired by the Unix `tree` command but focused on filtering by search term and maximum depth.

## Features

* Recursively scan directories up to a configurable depth
* Filter results by a case-insensitive search term
* Highlight matches in ANSI red color
* Show counts of matching subdirectories
* Simple, fast, and zero-cost abstractions in Rust

## Installation

1. Ensure you have Rust and Cargo installed. If not, install via [rustup](https://rustup.rs/).
2. Clone this repository:

   ```sh
   git clone https://github.com/DevElCuy/xtree.git
   cd xtree
   ```
3. Build the binary:

   ```sh
   cargo build --release
   ```
4. (Optional) Install globally:

   ```sh
   cargo install --path .
   ```

## Usage

```sh
xtree <search> [directory] [depth]
```

* `<search>`: (required) term to filter directory names
* `[directory]`: (optional) path to start from (default: current directory)
* `[depth]`: (optional) maximum recursion depth (default: 3)

### Examples

Scan current directory for folders containing "src":

```sh
xtree src
```

Scan `/home/user/projects` up to 5 levels deep for "config":

```sh
xtree config /home/user/projects 5
```

## Command-Line Options

* `--help`: Display help information
* `--version`: Show version number

## Contributing

Contributions are welcome! Please open issues or pull requests:

1. Fork the repository.
2. Create a feature branch (`git checkout -b feature-name`).
3. Commit your changes (`git commit -m "Add feature-name"`).
4. Push to the branch (`git push origin feature-name`).
5. Open a pull request.

Be sure to run `cargo fmt` and `cargo clippy` before submitting.

## License

This project is licensed under the MIT license. See [LICENSE](LICENSE) for details.
