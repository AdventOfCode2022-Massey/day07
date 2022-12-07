// This program is licensed under the "MIT License".
// Please see the file LICENSE in this distribution
// for license terms.

//! Advent of Code Day 7.  
//! Ben and Bart Massey 2022

use std::collections::HashMap;

use aoc::{*, reparse::*};

type Path = Vec<String>;

type Files = HashMap<String, usize>;
type DirTable = HashMap<Path, Files>;

fn parse_input() -> DirTable {
    let cd_root_re = Reparse::new(r"^\$ cd /$");
    let cd_up_re = Reparse::new(r"^\$ cd \.\.$");
    let cd_re = Reparse::new(r"^\$ cd ([a-z]+)$");
    let ls_re = Reparse::new(r"^\$ ls$");
    let dir_re = Reparse::new(r"^dir ([a-z]+)$");
    let file_re = Reparse::new(r"^([0-9]+) ([a-z.]+)$");

    let mut result: DirTable = HashMap::new();
    let mut cwd = Path::new();

    let mut insert_file = |cwd: &Path, name: String, size: usize| {
        if let Some(entry) = result.get_mut(cwd) {
            entry.insert(name, size);
        } else {
            let files: Files = [(name, size)].into_iter().collect();
            result.insert(cwd.clone(), files);
        }
    };

    for line in input_lines() {
        #[allow(clippy::if_same_then_else)]
        if cd_root_re.parse(&line).is_some() {
            cwd = Path::new();
        } else if cd_up_re.parse(&line).is_some() {
            assert!(cwd.pop().is_some());
        } else if let Some(fields) = cd_re.parse(&line) {
            let name: String = fields.get(1);
            cwd.push(name);
        } else if ls_re.parse(&line).is_some() {
            // no-op
        } else if dir_re.parse(&line).is_some() {
            // no-op
        } else if let Some(fields) = file_re.parse(&line) {
            let size: usize = fields.get(1);
            let name: String = fields.get(2);
            insert_file(&cwd, name, size);
        } else {
            panic!("unknown input line: {line}");
        }
    }

    result
}

fn dir_size(dirs: &DirTable, path: &Path) -> usize {
    dirs.get(path).unwrap().values().sum()
}

fn dir_total_size(dirs: &DirTable, path: &Path) -> usize {
    dirs
        .keys()
        .filter(|&p| p.starts_with(path))
        .map(|p| dir_size(dirs, p))
        .sum()
}

fn main() {
    let dirs = parse_input();
    match get_part() {
        Part1 => {
            let total: usize = dirs
                .keys()
                .filter_map(|p| {
                    let s = dir_total_size(&dirs, p);
                    if s <= 100_000 {
                        Some(s)
                    } else {
                        None
                    }
                })
                .sum();
            println!("{total}");
        }
        Part2 => todo!(),
    }
}
