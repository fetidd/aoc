use clap::Parser;
use reqwest::blocking::Client;
use reqwest::header::{COOKIE, USER_AGENT};
use std::fs;
use std::{collections::HashMap, path::PathBuf};

const AOC_URL: &str = "https://adventofcode.com";

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    #[arg(short, long, default_value_t = false)]
    new: bool,

    year: u32,
    day: u32,
    part: u8,

    #[arg(short, long)]
    path: Option<PathBuf>,
}

fn main() {
    let args = Args::parse();
    let path = match args.path {
        Some(path) => path,
        _ => std::env::current_dir().unwrap(),
    };
    let inputs = get_inputs(&path);
    if args.new {
        let mut code = format!(
            "pub fn run(input: &str, part: u8) -> String {{\n    \"{} {}\".into()\n}}\n\n",
            args.year, args.day
        );
        code.push_str("#[cfg(test)]\n");
        code.push_str("mod test {\n");
        code.push_str("    use super::*;\n\n");
        code.push_str("    #[test]\n");
        code.push_str("    fn test_run() {\n");
        code.push_str("        let input = \"\";\n");
        code.push_str("        assert_eq!(\"\", &run(input, 1));\n");
        code.push_str("    }\n");
        code.push_str("}\n");
        fs::write(
            path.join("src")
                .join(&format!("year{}", args.year))
                .join(&format!("day{}.rs", args.day)),
            code.as_bytes(),
        )
        .expect("failed to write new puzzle");
    } else {
        let input_str = {
            if let Some(input) = inputs.get(&(args.year as usize, args.day as usize)) {
                input.clone()
            } else {
                if let Ok(session_cookie) = fs::read_to_string(path.join("session_cookie.txt")) {
                    let client = Client::default();
                    let input_url = format!("{AOC_URL}/{}/day/{}/input", args.year, args.day);
                    println!("fetching input from {input_url}");
                    if let Ok(recv) = client
                        .get(input_url)
                        .header(COOKIE, session_cookie.trim())
                        .header(
                            USER_AGENT,
                            "Hi Eric! Just grabbing my input! - Ben Jones (fetiddius@gmail.com)",
                        )
                        .send()
                        .expect("failed requesting input")
                        .text()
                    {
                        if recv.contains("Please log in") {
                            panic!("Login to aoc failed: {recv}");
                        }
                        fs::write(
                            path.join("inputs")
                                .join(&format!("{}_{}", args.year, args.day)),
                            &recv,
                        )
                        .expect("failed to write downloaded input to file");
                        recv
                    } else {
                        panic!("failed to download input");
                    }
                } else {
                    panic!("missing session cookie");
                }
            }
        };
        let start = std::time::Instant::now();
        let res: String = aoc::get_puzzle(args.year, args.day)(&input_str, args.part);
        let end = std::time::Instant::now();
        println!("Answer: {res} ({:?})", end - start);
    }
}

fn get_inputs(path: &PathBuf) -> HashMap<(usize, usize), String> {
    let inputs_path = path.join("inputs");
    if !inputs_path.exists() {
        fs::create_dir(inputs_path).expect("failed to create missing inputs path");
    }
    let inputs_dir = fs::read_dir(path.join("inputs")).expect("failed to read inputs dir");
    let inputs = inputs_dir.into_iter().map(|dir_entry| {
        if let Ok(de) = dir_entry {
            let file_name = de
                .file_name()
                .into_string()
                .expect("failed to get filename!");
            // check that the filename is <year>_<day>?
            let file_parts: Vec<_> = file_name.split("_").collect();
            if file_parts.len() == 2 {
                let year = file_parts[0].parse::<usize>().unwrap();
                let day = file_parts[1].parse::<usize>().unwrap();
                let input_str =
                    fs::read_to_string(de.path()).expect("failed to read input data to string");
                ((year, day), input_str)
            } else {
                panic!("filename was incorrect: {file_name}");
            }
        } else {
            panic!("Something bad happened!")
        }
    });
    HashMap::from_iter(inputs)
}
