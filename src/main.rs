extern crate rayon;

use rayon::prelude::*;

use std::io;
use std::fs::{self, DirEntry};
use std::path::Path;
use std::env;

/**
 * compile: rustc du.rs
 * run: compare on mac
 * /usr/bin/time -lp ./du ~/Documents
 * /usr/bin/time -lp du -sk ~/Documents
 */

fn visit_dirs(dir: &Path, depth:u32) -> io::Result<u64> {
    let metadata = match dir.metadata() {
        Ok(metadata) => metadata,
        Err(_e) => {
            // no permissions, or a symlink
            // println!("error reading path {} - Reason {}", dir.display(), e);
            return Ok(0)
        }
    };

    let mut size = metadata.len();

    if metadata.is_dir() {
        let entries:Vec<DirEntry> = fs::read_dir(dir)?.into_iter().filter_map(|e| e.ok()).collect();

        let results:u64 = entries.par_iter().map(|entry| {
            let path = entry.path();

            match visit_dirs(&path, depth + 1) {
                Ok(child_size) => {
                    child_size
                },
                Err(_) => 0
            }
        }).sum();

        size += results;
    }

    Ok(size)
}

fn friendly(bytes:u64) -> String  {
    return format!("{:>8.3}K", bytes as f64 / 1024.0);
}

fn main() {
    let args:Vec<String> =  env::args().collect();
    let len = args.len();
    let path = if len > 1 { &args[1] } else { "./" };
    println!("Scanning path {}...", path);
    let size = visit_dirs(Path::new(path), 0).unwrap();

    println!("Space scan done: {}", friendly(size));
}