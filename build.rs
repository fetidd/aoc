use std::env;
use std::fs;
use std::path::Path;

fn main() {
    let out_dir = env::var("OUT_DIR").unwrap();
    let dest_path = Path::new(&out_dir).join("days.rs");
    let src_dir = Path::new("src");
    let mut days: Vec<(u32, String)> = Vec::new();
    for entry in fs::read_dir(src_dir).expect("Could not read src dir") {
        let entry = entry.expect("Could not read entry");
        let path = entry.path();
        if path.is_file() && path.extension().map_or(false, |ext| ext == "rs") {
            if let Some(stem) = path.file_stem().and_then(|s| s.to_str()) {
                if stem.starts_with("day") {
                    if let Ok(num) = stem["day".len()..].parse::<u32>() {
                        days.push((num, stem.to_string()));
                    }
                }
            }
        }
    }
    days.sort_by_key(|k| k.0);
    let mut code = String::new();
    let manifest_dir = env::var("CARGO_MANIFEST_DIR").unwrap();
    let abs_root_path = Path::new(&manifest_dir);
    for (_, name) in &days {
        let path_str = abs_root_path.join("src").join(&format!("{}.rs", name));
        let path_str = path_str.to_str().unwrap();
        code.push_str(&format!("#[path = \"{}\"]\n", path_str)); // ensure the compiler checks next to main.rs for the day modules
        code.push_str(&format!("pub mod {};\n", name));
    }
    code.push_str("\n");
    code.push_str("pub fn get_day(day: u32) -> fn(&str) -> String {\n");
    code.push_str("    match day {\n");
    for (num, name) in &days {
        code.push_str(&format!("        {} => {}::run,\n", num, name));
    }
    code.push_str("        _ => panic!(\"Day {} not implemented!\", day)\n");
    code.push_str("    }\n");
    code.push_str("}\n");
    fs::write(&dest_path, code).expect("failed to write code");
    println!("cargo:rerun-if-changed=src");
}
