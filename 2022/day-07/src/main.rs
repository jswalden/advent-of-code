use std::collections::{HashMap, HashSet};
use std::iter::Peekable;
use std::str::Lines;

#[derive(Debug)]
struct File {
    size: usize,
}

type DirectoryContents = HashMap<String, Entry>;

#[derive(Debug)]
struct Directory {
    contents: DirectoryContents,
    size: usize,
}

#[derive(Debug)]
enum Entry {
    Dir(Directory),
    File(File),
}

fn parse_directory_listing(
    lines: &mut Peekable<Lines>,
) -> (DirectoryContents, HashSet<String>, usize) {
    let mut dir_contents = DirectoryContents::new();
    let mut unconsumed_subdirs = HashSet::<String>::new();
    let mut total_files_size = 0;

    // Parse directory contents.
    loop {
        let peeked_line = match lines.peek() {
            None => break,
            Some(peeked_line) => *peeked_line,
        };

        // Peek at next line without consuming it in case it's not directory contents.
        let mut toks = peeked_line.split(' ');

        let first_tok = if let Some(tok) = toks.next() {
            tok
        } else {
            // Consume the empty line.
            lines.next();
            break;
        };

        if first_tok == "$" {
            // Command.  Continue to in-directory command parsing without
            // consuming the line.
            break;
        }

        // Definitely directory contents, so consume the line.
        lines.next();

        if first_tok == "dir" {
            // Add it to directory contents, continue next contents.
            let dirname = toks.next().expect("directory name");
            let freshly_inserted = unconsumed_subdirs.insert(dirname.to_owned());
            assert!(freshly_inserted, "shouldn't have duplicative entry");
        } else {
            let filesz = first_tok.parse::<usize>().expect("size");
            total_files_size += filesz;
            let filename = toks.next().expect("filename");
            dir_contents.insert(filename.to_owned(), Entry::File(File { size: filesz }));
        }
    } // processing directory contents listing

    (dir_contents, unconsumed_subdirs, total_files_size)
}

fn parse_directory(lines: &mut Peekable<Lines>) -> Directory {
    let mut dir_entries = None;
    let mut contained_dirs_size = 0;
    let mut total_files_size = 0;

    // Run commands within directory.
    loop {
        let peeked_line = match lines.peek() {
            None => break,
            Some(peeked_line) => *peeked_line,
        };

        lines.next();
        let mut toks = peeked_line.split(' ');

        let first_tok = if let Some(tok) = toks.next() {
            tok
        } else {
            // Consume an empty line.
            break;
        };

        assert!(first_tok == "$", "expect command");

        match toks.next().expect("command") {
            "ls" => {
                let (contents, unconsumed_subdirs, files_size) = parse_directory_listing(lines);
                dir_entries = Some((contents, unconsumed_subdirs));
                total_files_size = files_size;
                continue;
            }
            "cd" => {
                let cd_name = toks.next().expect("cd <dirname>");
                if cd_name == ".." {
                    // Done with this directory.
                    break;
                }

                let subdir = parse_directory(lines);
                if let Some((ref mut dir_contents, ref mut unconsumed_subdirs)) = dir_entries {
                    assert!(
                        unconsumed_subdirs.contains(cd_name),
                        "should have seen dir already"
                    );

                    contained_dirs_size += subdir.size;
                    unconsumed_subdirs.remove(cd_name);
                    dir_contents.insert(cd_name.to_owned(), Entry::Dir(subdir));
                } else {
                }
            }
            cmd => {
                panic!("unexpected command: {}", cmd);
            }
        }
    } // Running commands within directory

    // Left directory, return the directory.
    let (dir_entries, unconsumed_subdirs) = dir_entries.expect("dir_entries");
    assert!(
        unconsumed_subdirs.is_empty(),
        "should have recurred into all subdirs at this point"
    );

    Directory {
        contents: dir_entries,
        size: contained_dirs_size + total_files_size,
    }
}

fn parse_input() -> (String, Directory) {
    let contents = include_str!("../input");

    let mut lines = contents.lines().peekable();

    let cd_line = lines.next().expect("first line cd into top");

    let mut cd_toks = cd_line.split(' ');

    let dollar = cd_toks.next().expect("$");
    assert!(dollar == "$");

    let cd = cd_toks.next().expect("cd");
    assert!(cd == "cd");

    let cd_dir = cd_toks.next().expect("cd_dir");
    (cd_dir.to_owned(), parse_directory(&mut lines))
}

fn sum_sizes_up_to_100k(dir: &Directory) -> usize {
    const MAX: usize = 100_000;

    let mut sum_of_100k_sizes = 0;
    if dir.size <= MAX {
        sum_of_100k_sizes += dir.size;
    }

    for (_, entry) in dir.contents.iter() {
        match entry {
            Entry::File(_) => continue,
            Entry::Dir(nested) => {
                let nested_up_to_100k = sum_sizes_up_to_100k(nested);
                sum_of_100k_sizes += nested_up_to_100k;
            }
        }
    }

    sum_of_100k_sizes
}

const TOTAL_DISK_SPACE: usize = 70_000_000;
const UNUSED_SPACE_REQD: usize = 30_000_000;

fn smallest_directory_at_least(dir: &Directory, size: usize) -> usize {
    let mut best_size = usize::MAX;

    macro_rules! update {
        ($val:expr) => {
            let v = $val;
            if v >= size && v < best_size {
                best_size = v;
            }
        };
    }

    update!(dir.size);

    for (_, entry) in dir.contents.iter() {
        match entry {
            Entry::File(_) => continue,
            Entry::Dir(nested) => {
                update!(smallest_directory_at_least(nested, size));
            }
        }
    }

    best_size
}

fn main() {
    let (name, dir) = parse_input();

    let sum_of_100k_sizes = sum_sizes_up_to_100k(&dir);
    println!("sum of sizes up to 100k in {}: {}", name, sum_of_100k_sizes);

    let dir_size = dir.size;
    println!("directory size: {}", dir_size);

    let unused_size = TOTAL_DISK_SPACE - dir_size;
    println!("unused space: {}", unused_size);

    let space_to_free = UNUSED_SPACE_REQD - unused_size;
    println!("minimum space to be freed: {}", space_to_free);

    match smallest_directory_at_least(&dir, space_to_free) {
        usize::MAX => {
            println!("no directory found of at least {} size", space_to_free);
        }
        smallest_dir_to_remove_size => {
            println!(
                "size of smallest dir to remove: {}",
                smallest_dir_to_remove_size
            );
        }
    }
}
