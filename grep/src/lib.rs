extern crate regex;

use failure::Error;
use regex::Regex;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;

/// While using raw slice of str to handle flags is convenient,
/// in the real-world projects it is customary to use a struct,
/// that contains flags-related logic. So in this exercise
/// we ask you to implement a custom struct.
///
/// If you are curious about real-world implementation, refer to the `clap-rs` crate:
/// https://github.com/kbknapp/clap-rs/blob/master/src/args/arg_matches.rs
#[derive(Debug, Default)]
pub struct Flags {
    line_number: bool,
    name_only: bool,
    case_insensitive: bool,
    invert_match: bool,
    line_match: bool,
}

impl Flags {
    pub fn new(flags: &[&str]) -> Self {
        let mut f = Flags {
            ..Default::default()
        };
        for &flag in flags {
            match flag {
                "-n" => f.line_number = true,
                "-l" => f.name_only = true,
                "-i" => f.case_insensitive = true,
                "-v" => f.invert_match = true,
                "-x" => f.line_match = true,
                _ => (),
            }
        }
        f
    }
}

pub fn grep(pattern: &str, flags: &Flags, files: &[&str]) -> Result<Vec<String>, Error> {
    let mut pattern = pattern.to_string();
    if flags.case_insensitive {
        pattern.insert_str(0, "(?i)")
    }

    let re = Regex::new(pattern.as_str())?;

    let mut res = Vec::new();

    for file in files {
        let p = Path::new(file);
        let f = File::open(p)?;
        let f = BufReader::new(f);

        for (n, l) in f.lines().enumerate() {
            let l = l?;
            match find_match(&re, flags, &p, &l, n + 1, files.len() > 1) {
                Some(s) => res.push(s),
                None => (),
            }
        }
    }

    if files.len() > 1 && flags.name_only {
        res.dedup();
    }

    Ok(res)
}

fn find_match(
    re: &Regex,
    flags: &Flags,
    path: &Path,
    line: &str,
    line_no: usize,
    print_filename: bool,
) -> Option<String> {
    let m = re.find(line);

    if let Some(m) = m {
        if flags.line_match && m.as_str() != line {
            return None;
        }
    }

    if (m.is_some() && !flags.invert_match) || (m.is_none() && flags.invert_match) {
        let mut res = String::new();

        if flags.name_only {
            res.push_str(path.file_name()?.to_str()?);
        } else {
            if print_filename {
                res.push_str(path.file_name()?.to_str()?);
                res.push(':');
            }
            if flags.line_number {
                res.push_str(&line_no.to_string());
                res.push(':');
            }
            res.push_str(line);
        }

        return Some(res);
    }

    None
}
