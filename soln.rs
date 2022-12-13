// This program is licensed under the "MIT License".
// Please see the file LICENSE in this distribution
// for license terms.

//! Advent of Code Day 7.  
//! Ben and Bart Massey 2022

use std::collections::HashMap;

use aoc::{*, reparse::*};

/// A `Path` is a sequence of path segments.
type Path = Vec<String>;

/// A directory contains a list of `Files` with a given size.
type Files = HashMap<String, usize>;

/// The `DirTable` holds all the known directories with
/// their known contents. There is no explicit relation
/// between them, but their paths are related.
type DirTable = HashMap<Path, Files>;

/// Take the problem input to a [DirTable] representing the
/// structure to be analyzed.
fn parse_input() -> DirTable {
    // Regexes for parsing various lines in the input.
    // All of these regexes are disjoint.
    let cd_root_re = Reparse::new(r"^\$ cd /$");
    let cd_up_re = Reparse::new(r"^\$ cd \.\.$");
    let cd_re = Reparse::new(r"^\$ cd ([a-z]+)$");
    let ls_re = Reparse::new(r"^\$ ls$");
    let dir_re = Reparse::new(r"^dir ([a-z]+)$");
    let file_re = Reparse::new(r"^([0-9]+) ([^ ]+)$");

    // Allocate the resulting `DirTable` to be returned, and
    // start the "current working directory" at the root.
    let mut result: DirTable = HashMap::new();
    let mut cwd = Path::new();

    // Insert the file with given `name` and `size` into the
    // directory at the given `path`, creating a new
    // `result` directory entry for `path` if needed.
    let mut insert_file = |path: &Path, name: String, size: usize| {
        if let Some(entry) = result.get_mut(path) {
            // We may visit a file for a subsequent time,
            // in which case the file size should be identical
            // to previous visits.
            if let Some(&old_size) = entry.get(&name) {
                assert_eq!(size, old_size);
            } else {
                entry.insert(name, size);
            }
        } else {
            // Make a new directory with the given file in it.
            let files: Files = [(name, size)].into_iter().collect();
            result.insert(path.clone(), files);
        }
    };

    // Parse the lines of the input.
    for line in input_lines() {
        #[allow(clippy::if_same_then_else)]
        if cd_root_re.parse(&line).is_some() {
            // Drop the existing path and go back to the root.
            cwd = Path::new();
        } else if cd_up_re.parse(&line).is_some() {
            // Drop the last component of the existing cwd.
            // Will panic if going above the root is
            // attempted.
            assert!(cwd.pop().is_some());
        } else if let Some(fields) = cd_re.parse(&line) {
            // Add a new named component to the end of cwd.
            let name: String = fields.get(1);
            cwd.push(name);
        } else if ls_re.parse(&line).is_some() {
            // Ignore `ls` lines as they have no useful information.
        } else if dir_re.parse(&line).is_some() {
            // Ignore `dir` lines as they have no useful information.
        } else if let Some(fields) = file_re.parse(&line) {
            // Insert the given file into the structure.
            let size: usize = fields.get(1);
            let name: String = fields.get(2);
            insert_file(&cwd, name, size);
        } else {
            panic!("unknown input line: {line}");
        }
    }

    result
}

/// Find the size of the directory at the given `path` in
/// `dirs`.  This is the total size of the contained files,
/// excluding any subdirectories.
fn dir_size(dirs: &DirTable, path: &Path) -> usize {
    dirs.get(path).unwrap().values().sum()
}

/// Find the total size of the directory at the given `path`
/// in `dirs` and its subdirectories, recursively. A
/// directory *d* is a subdirectory of the directory at
/// `path` if *d*'s path starts with `path`. (Note that
/// `path` starts with `path`.)
fn dir_total_size(dirs: &DirTable, path: &Path) -> usize {
    dirs
        .keys()
        .filter(|&p| p.starts_with(path))
        .map(|p| dir_size(dirs, p))
        .sum()
}

/// Select all the paths with legal total size
/// and sum their sizes.
fn solve_part1(dirs: &DirTable, max: usize) -> usize {
    dirs
        .keys()
        .filter_map(|p| {
            let s = dir_total_size(dirs, p);
            if s <= max {
                Some(s)
            } else {
                None
            }
        })
        .sum()
}


#[cfg(test)]
mod test {
    use super::*;

    macro_rules! path {
        ($($seg:literal),*) => {
            vec![$($seg.to_string()),*]
        };
    }

    macro_rules! file {
        ($name:literal, $value:literal) => {
            ($name.to_string(), $value)
        };
    }

    fn make_dir_table() -> DirTable {
        let t = [
            (path![], vec![
                file!{"a0f0", 10_000},
            ]),
            (path!["a0"], vec![
                file!{"a0f0", 9_999},
                file!{"a0f1", 1},
            ]),
            (path!["a0", "a1"], vec![
                file!{"a1f0", 9_998},
                file!{"a1f1", 2},
            ]),
            (path!["a0", "a1", "a2"], vec![
                file!{"a1f0", 9_997},
                file!{"a1f1", 3},
            ]),
            (path!["a0", "a1", "b2"], vec![
                file!{"a1f0", 9_996},
                file!{"a1f1", 4},
            ]),
        ];
        t
            .into_iter()
            .map(|(path, files)| (path, files.into_iter().collect()))
            .collect()
    }

    #[test]
    fn test_dir_size() {
        let t = make_dir_table();
        assert_eq!(10_000, dir_size(&t, &path![]));
        assert_eq!(10_000, dir_size(&t, &path!["a0"]));
        assert_eq!(10_000, dir_size(&t, &path!["a0", "a1"]));
        assert_eq!(10_000, dir_size(&t, &path!["a0", "a1", "a2"]));
        assert_eq!(10_000, dir_size(&t, &path!["a0", "a1", "b2"]));
    }

    #[test]
    fn test_dir_total_size() {
        let t = make_dir_table();
        assert_eq!(50_000, dir_total_size(&t, &path![]));
        assert_eq!(40_000, dir_total_size(&t, &path!["a0"]));
        assert_eq!(30_000, dir_total_size(&t, &path!["a0", "a1"]));
        assert_eq!(10_000, dir_total_size(&t, &path!["a0", "a1", "a2"]));
        assert_eq!(10_000, dir_total_size(&t, &path!["a0", "a1", "b2"]));
    }

    #[test]
    fn test_solve_part1() {
        let t = make_dir_table();
        assert_eq!(140_000, solve_part1(&t, 50_000));
        assert_eq!(90_000, solve_part1(&t, 40_000));
        assert_eq!(20_000, solve_part1(&t, 10_000));
    }
}

#[cfg(feature = "logging")]
/// Show the given `dirs` and their files in
/// lexical-alphabetical order.
fn print_dir_table(dirs: &DirTable) {
    let mut paths: Vec<&Path> = dirs.keys().collect();
    paths.sort();
    for &p in &paths {
        println!("{}", p.join("/"));
        let fs = dirs.get(p).unwrap();
        for (name, size) in fs.iter() {
            println!("    {size} {name}");
        }
        let np = p.len();
        for sd in &paths {
            if sd.len() == np + 1 && sd.starts_with(p) {
                println!("    dir {}", sd.last().unwrap());
            }
        }
    }
}

#[cfg(feature = "logging")]
/// Show the given `dirs` and their total sizes in
/// lexical-alphabetical order.
fn print_total_sizes(dirs: &DirTable) {
    let mut paths: Vec<&Path> = dirs.keys().collect();
    paths.sort();
    for p in paths {
        println!("{} {}", p.join("/"), dir_total_size(dirs, p));
    }
}

fn main() {
    let dirs = parse_input();
    #[cfg(feature = "logging")] {
        print_dir_table(&dirs);
        println!();
        print_total_sizes(&dirs);
        println!();
    }
    match get_part() {
        Part1 => {
            let total = solve_part1(&dirs, 100_000);
            println!("{total}");
        }
        Part2 => todo!(),
    }
}
