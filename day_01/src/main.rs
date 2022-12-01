use common::{get_arg, read_file_to_string};

fn main() -> Result<(), anyhow::Error> {
    let input_file_path = get_arg(1, "pass path to input file as first argument");
    let input_string = read_file_to_string(&input_file_path)?;

    println!("{}", input_string.len());

    Ok(())
}
