use clap::Parser;
use std::fs;
use std::{collections::HashMap, path::PathBuf};

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    #[arg(short, long, default_value_t = false)]
    new: bool,

    year: u32,
    day: u32,

    #[arg(short, long)]
    path: Option<PathBuf>,
}

fn main() {
    let args = Args::parse();
    let path = match args.path {
        Some(path) => path,
        None => std::env::current_dir().unwrap()
    };
    let inputs = get_inputs(&path);
    if args.new {
        let mut code = format!(
            "pub fn run(input: &str) -> String {{\n    \"{} {}\".into()\n}}\n\n",
            args.year, args.day
        );
        code.push_str("#[cfg(test)]\n");
        code.push_str("mod test {\n");
        code.push_str("    use super::*;\n\n");
        code.push_str("    #[test]\n");
        code.push_str("    fn test_run() {\n");
        code.push_str("        let input = \"\";\n");
        code.push_str("        assert_eq!(\"\", &run(input));\n");
        code.push_str("    }\n");
        code.push_str("}\n");
        fs::write(
            path.join("src").join(&format!("year{}", args.year)).join(&format!("day{}.rs", args.day)),
            code.as_bytes(),
        )
        .expect("failed to write new puzzle");
    } else {
        let run_fn = aoc::get_puzzle(args.year, args.day);
        if let Some(input) = inputs.get(&(args.year as usize, args.day as usize)) {
            let res: String = run_fn(input);
            println!("Answer: {res}");
        } else {
            println!("Failed to find input for this puzzle");
        }
    }
}

fn get_inputs(path: &PathBuf) -> HashMap<(usize, usize), String> {
    let inputs_dir = fs::read_dir(path.join("inputs")).expect("failed to read inputs dir; have you added them?");
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
                let input_str = fs::read_to_string(de.path()).expect("failed to read input data to string");
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
