use std::fs;
use std::path::Path;
use clap::{App, Arg};

/// A simple tree structure representing a directory and its matching subdirectories.
#[derive(Debug)]
struct Tree {
    name: String,
    children: Vec<Tree>,
}

/// Recursively scans a directory (up to `max_depth`) and builds a list of children along with a score.
///
/// For each directory:
/// - It recurses only when `depth < max_depth`.
/// - Adds 1 to the score if the directory name (case‑insensitive) contains the search term,
///   plus the scores of any matching descendants.
/// - Only includes directories that either match or have matching descendants.
fn scan_dir(path: &Path, depth: usize, max_depth: usize, searchterm_lower: &str) -> (Vec<Tree>, u32) {
    if depth >= max_depth {
        return (Vec::new(), 0);
    }

    let mut total_score = 0;
    let mut children = Vec::new();

    if let Ok(entries) = fs::read_dir(path) {
        for entry in entries.flatten() {
            if let Ok(file_type) = entry.file_type() {
                if file_type.is_dir() {
                    let name = entry.file_name().to_string_lossy().into_owned();
                    let name_lower = name.to_lowercase();
                    let (child_children, child_score) =
                        scan_dir(&entry.path(), depth + 1, max_depth, searchterm_lower);
                    let found = name_lower.contains(searchterm_lower);
                    // If the directory name contains the term, count it.
                    let score_here = if found { 1 } else { 0 };

                    // Only include this directory if it or one of its descendants matches.
                    if found || child_score > 0 {
                        children.push(Tree {
                            name,
                            children: child_children,
                        });
                    }
                    total_score += score_here + child_score;
                }
            }
        }
    }
    (children, total_score)
}

/// Builds the filtered directory tree starting at `dirpath`.
///
/// Returns `None` if no directory (including subdirectories) matches.
fn build_tree_dict(dirpath: &str, searchterm_lower: &str, max_depth: usize) -> Option<Tree> {
    let path = Path::new(dirpath);
    let (children, score) = scan_dir(path, 0, max_depth, searchterm_lower);
    if score == 0 {
        None
    } else {
        Some(Tree {
            name: dirpath.to_string(),
            children,
        })
    }
}

/// Highlights the first occurrence of `substr` in `s` with ANSI red color.
/// Assumes ASCII so that byte indices match character boundaries.
fn highlight_substring(s: &str, substr_lower: &str) -> String {
    let s_lower = s.to_lowercase();
    if let Some(pos) = s_lower.find(substr_lower) {
        let before = &s[..pos];
        let matched = &s[pos..pos + substr_lower.len()];
        let after = &s[pos + substr_lower.len()..];
        format!("{}{}{}{}{}", before, "\x1b[91m", matched, "\x1b[0m", after)
    } else {
        s.to_owned()
    }
}

/// Recursively prints the tree structure with branch lines.
///
/// - `skip_first`: if true, the current level isn’t printed (used for the root).
/// - `count`: if false (i.e. at the top‐level call), the total matching directories count is printed.
fn print_tree(tree: &Tree, searchterm_lower: &str, prefix: &str, skip_first: bool, count: bool) -> u32 {
    let mut dir_count = 0;
    let num_children = tree.children.len();

    for (i, child) in tree.children.iter().enumerate() {
        let is_last = i == num_children - 1;
        let branch = if is_last { "└── " } else { "├── " };
        let next_prefix = if skip_first {
            prefix.to_string()
        } else if is_last {
            format!("{}    ", prefix)
        } else {
            format!("{}│   ", prefix)
        };

        if !skip_first {
            let child_name_lower = child.name.to_lowercase();
            let display_name = if child_name_lower.contains(searchterm_lower) {
                dir_count += 1;
                highlight_substring(&child.name, searchterm_lower)
            } else {
                child.name.clone()
            };
            println!("{}{}{}", prefix, branch, display_name);
        }

        let child_prefix = if skip_first { prefix.to_string() } else { next_prefix };
        dir_count += print_tree(child, searchterm_lower, &child_prefix, false, true);
    }

    if !count {
        println!(
            "\n{} {}",
            dir_count,
            if dir_count == 1 { "directory" } else { "directories" }
        );
    }
    dir_count
}

fn main() {
    // Clone the app so that we can later print the help message.
    let mut app = App::new("xtree")
        .version("0.1")
        .about("A lightweight directory tree generator.")
        .arg(
            Arg::new("search")
                .help("Search term to filter directory names (if provided)")
                .required(false)
                .index(1),
        )
        .arg(
            Arg::new("directory")
                .help("Directory to generate tree from (default: current directory)")
                .required(false)
                .index(2),
        )
        .arg(
            Arg::new("depth")
                .short('d')
                .long("depth")
                .help("Maximum depth of directory tree (default: 3)")
                .takes_value(true)
                .default_value("3"),
        );

    let matches = app.clone().get_matches();

    let search = matches.value_of("search").unwrap_or("");
    if search.is_empty() {
        app.print_help().expect("Failed to print help");
        println!();
        return;
    }

    let directory = matches.value_of("directory").unwrap_or(".");
    let depth: usize = matches
        .value_of("depth")
        .unwrap_or("3")
        .parse()
        .unwrap_or(3);
    // Precompute the lower-case version of the search term.
    let search_lower = search.to_lowercase();

    if let Some(tree) = build_tree_dict(directory, &search_lower, depth) {
        // Print the root directory.
        println!("{}", tree.name);
        // Print the rest of the tree.
        print_tree(&tree, &search_lower, "", false, false);
    } else {
        println!("No directories match the search term.");
    }
}
