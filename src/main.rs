use std::fs::{self, File, exists};
use std::io::prelude::*;
use std::path::Path;

fn main() -> std::io::Result<()> {
    let base_path = Path::new("data");
    fs::create_dir_all(base_path)?;

    let file_path = base_path.join("output.txt");
    if file_path.exists() {
        let mut file = File::open(file_path)?;
        let mut contents = String::new();
        file.read_to_string(&mut contents)?;
    } else {
        let mut file = File::create(file_path)?;
        file.write_all(b"skibidi string")?;
    }
    Ok(())
}
