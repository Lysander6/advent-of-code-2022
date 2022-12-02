use std::{
    env,
    fs::File,
    io::{BufReader, Read},
    path::Path,
};

use anyhow::{Context, Result};

pub fn get_arg(nth: usize) -> Result<String> {
    env::args()
        .nth(nth)
        .with_context(|| format!("trying to read nth ({}) argument", nth))
}

fn read_file(path_str: &str) -> Result<BufReader<File>> {
    let path = Path::new(path_str);
    let file = File::open(path).with_context(|| format!("trying to open {}", path.display()))?;

    Ok(BufReader::new(file))
}

pub fn read_file_to_string(path_str: &str) -> Result<String> {
    let mut buf = read_file(path_str)?;
    let mut s = String::new();

    buf.read_to_string(&mut s)?;

    Ok(s)
}
