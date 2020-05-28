use regex::Regex;
use std::env;
use std::fs;
use std::path::{Path, PathBuf};

fn main() {
    let functions_dir = PathBuf::from("./src/builtins/functions/");
    println!("{:?}", env::current_dir().unwrap());

    let out_dir = env::var_os("OUT_DIR").unwrap();
    let out_path = Path::new(&out_dir).join("functions.rs");
    let mut output = String::new();

    let re = Regex::new(r"fn (.+?)\(").unwrap();

    let func_names: Vec<_> = fs::read_dir(functions_dir)
        .unwrap()
        .map(|e| {
            let e = e.unwrap().path();

            if e.is_file() {
                let mod_name = e.file_stem().unwrap().to_str().unwrap();
                if mod_name == "mod" {
                    return vec![];
                }

                let mod_contents = fs::read_to_string(&e).unwrap();

                re.captures_iter(&mod_contents)
                    .map(|caps| caps.get(1).map(|s| s.as_str().to_owned()))
                    .flatten()
                    .collect()
            } else {
                vec![]
            }
        })
        .flatten()
        .collect();

    output.push_str("pub const FUNCTIONS: &[&dyn BuiltInCallable] = &[\n");
    for i in func_names {
        output.push_str(&format!("&{},\n", i));
    }
    output.push_str("];");

    println!("{}", output);
    fs::write(&out_path, &output).unwrap();
}
