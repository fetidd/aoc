use std::env;
use std::fs;
use std::path::Path;

fn main() {
    let out_dir = env::var("OUT_DIR").unwrap();
    let lib_dest_path = Path::new(&out_dir).join("lib.rs");
    let src_dir = Path::new("src");
    let manifest_dir = env::var("CARGO_MANIFEST_DIR").unwrap();
    let abs_root_path = Path::new(&manifest_dir);
    let mut years: Vec<(u32, Vec<(u32, String)>)> = Vec::new();
    if let Ok(src) = fs::read_dir(src_dir) {
        for entry in src {
            if let Ok(entry) = entry && entry.file_name().to_str().is_some_and(|f| f.starts_with("year")) {
                let year = (&entry.file_name().to_str().unwrap()[4..]).parse::<u32>().expect("failed parsing year");
                let mut days: Vec<(u32, String)> = Vec::new();
                if let Ok(year_dir) = fs::read_dir(src_dir.join(&format!("year{year}"))) {
                    for year_entry in year_dir {
                        if let Ok(year_entry) = year_entry &&
                            year_entry.file_name().to_str().is_some_and(|f| f.starts_with("day")) &&
                            year_entry.file_name().to_str().is_some_and(|f| f.ends_with(".rs"))
                        {
                            if let Some(day_stem) = year_entry.path().file_stem() {
                                let day_str = day_stem.to_string_lossy();
                                let day = (&day_str[3..]).parse::<u32>().unwrap();
                                days.push((day, day_str.to_string()));
                            }
                        }
                    }
                }
                years.push((year, days));
            }
        }
    }
    let mut mods = String::new();
    let mut function = String::from("pub fn get_puzzle(year: u32, day: u32) -> fn(&str) -> String {\n    match (year, day) {\n");
    for (year, days) in years {
        let path_str = abs_root_path.join(&format!("year{year}"));
        let path_str = path_str.to_str().unwrap();
        mods.push_str(&format!("#[path = \"{}\"]\n", path_str)); // ensure the compiler checks next to main.rs for the day modules
        mods.push_str(&format!("pub mod year{year} {{\n"));
        for (day, file_name) in days {
            function.push_str(&format!("        ({year}, {day}) => year{year}::day{day}::run,\n"));
            let path_str = abs_root_path.join("src").join(&format!("year{year}")).join(file_name+".rs");
            let path_str = path_str.to_str().unwrap();
            mods.push_str(&format!("#[path = \"{}\"]\n", path_str)); // ensure the compiler checks next to main.rs for the day modules
            mods.push_str(&format!("    pub mod day{day};\n"));
        }
        mods.push_str("}\n");
    }
    function.push_str("        _ => panic!(\"Invalid selection: {year} {day}\")\n    }\n}\n");
    let code = format!("{mods}\n\n{function}");
    fs::write(&lib_dest_path, code).expect("failed to write code");
    println!("cargo:rerun-if-changed=src");
}
