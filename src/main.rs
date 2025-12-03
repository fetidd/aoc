include!(concat!(env!("OUT_DIR"), "/days.rs"));

use clap::Parser;
use std::fs::{self, File};
use std::{collections::HashMap, path::PathBuf};

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    #[arg(short, long, default_value_t = 0)]
    new: u32,

    #[arg(short, long, default_value_t = 0)]
    run: u32,

    #[arg(short, long, default_value = "/home/ben/AoC25")]
    path: PathBuf,

    #[arg(long, default_value_t = false)]
    force: bool,
}

fn main() {
    let args = Args::parse();
    let inputs = get_inputs(&args.path);
    if args.run > 0 {
        let run_fn = get_day(args.run);
        let res: String = run_fn(&inputs[&(args.run as usize)]);
        println!("Answer: {res}");
    } else if args.new > 0 {
        if inputs.contains_key(&(args.new as usize)) && !args.force {
            panic!("must --force to overwrite an existing day!");
        }
        let mut code = format!(
            "pub fn run(input: &str) -> String {{\n    todo!(\"day {}\")\n}}\n\n",
            args.new
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
            args.path.join("src").join(&format!("day{}.rs", args.new)),
            code.as_bytes(),
        )
        .expect("failed to write new day");
        File::create_new(args.path.join("inputs").join(&format!("day{}", args.new)))
            .expect("failed to create new input file");
    }
}

fn get_inputs(path: &PathBuf) -> HashMap<usize, String> {
    let inputs_dir = fs::read_dir(path.join("inputs")).expect("failed to read inputs dir");
    let inputs = inputs_dir.into_iter().map(|dir_entry| {
        if let Ok(de) = dir_entry {
            let mut file_name = de
                .file_name()
                .into_string()
                .expect("failed to get filename!");
            // check that the filename is dayN?
            let day = file_name
                .split_off(3)
                .parse::<usize>()
                .expect("failed to parse day number from filename");
            let input_str =
                fs::read_to_string(de.path()).expect("failed to read input data to string");
            (day, input_str)
        } else {
            panic!("Something bad happened!")
        }
    });
    HashMap::from_iter(inputs)
}
