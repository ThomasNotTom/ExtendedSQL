use std::env;
use std::fs;
use std::path::Path;

fn main() {
    let out_dir = env::var("OUT_DIR").unwrap();
    let dest_path = Path::new(&out_dir).join("config.rs");

    fs::write(&dest_path, "pub const CONFIG: &str = \"example_config\";")
        .expect("Unable to write file");

    println!("cargo:rerun-if-changed=build.rs");
}
