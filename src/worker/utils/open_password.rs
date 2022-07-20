use std::{io::Read, path::Path};

pub fn read_password_file(path: &str) -> Result<String, std::io::Error> {
    let file_path = Path::new(path);
    // println!("{}", file_path.display());
    let mut file = std::fs::File::open(file_path)?;
    let mut contents = String::new();
    match file.read_to_string(&mut contents) {
        Ok(_) => Ok(contents),
        Err(e) => Err(e),
    }
}
