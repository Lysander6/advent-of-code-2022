use std::{
    env,
    fs::File,
    io::{BufReader, Read},
    path::Path,
};

use anyhow::Result;

pub fn get_arg(nth: usize, err_msg: &str) -> String {
    env::args().nth(nth).expect(err_msg)
}

fn read_file(path_str: &str) -> BufReader<File> {
    let path = Path::new(path_str);
    let file = match File::open(path) {
        Ok(file) => file,
        Err(why) => panic!("couldn't open {}: {}", path.display(), why),
    };

    BufReader::new(file)
}

pub fn read_file_to_string(path_str: &str) -> Result<String> {
    let mut buf = read_file(path_str);
    let mut s = String::new();

    buf.read_to_string(&mut s)?;

    Ok(s)
}
