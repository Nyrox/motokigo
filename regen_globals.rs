//! ```cargo
//! [dependencies]
//! regex = "1"
//! ```
extern crate regex;

use regex::Regex;
use std::env;
use std::fs;
use std::process::Command;
use std::path::{Path, PathBuf};
use std::fs::{File, remove_file};

fn main() {
    let path_str = format!("{}\\src\\builtins\\functions\\functions.rs", env::current_dir().unwrap().to_str().unwrap());
    let out_path = Path::new(&path_str);
    
    fs::write(&out_path, "pub const FUNCTIONS: &[&dyn BuiltInCallable] = &[]").unwrap();

    let mut output = String::new();

    let expanded = 
        String::from_utf8(Command::new("cargo")
            .arg("expand")
            .arg("builtins::functions")
            .output()
            .expect("Build script failed bruh cringe")
            .stdout).unwrap();

    let re = Regex::new(r"struct\s+(.+?)[\s;{\(]").unwrap();
    let func_names = re.captures_iter(&expanded)
        .map(|caps| caps.get(1).map(|s| s.as_str().to_owned()))
        .flatten()
        .collect::<Vec<_>>();

    output.push_str("pub const FUNCTIONS: &[&dyn BuiltInCallable] = &[\n");
    for i in func_names {
        output.push_str(&format!("\t&{},\n", i));
    }
    output.push_str("];");
    fs::write(&out_path, &output).unwrap();
}
