use std::fs::{write, OpenOptions};
use std::io::{prelude::*, BufReader, Result};

fn main() -> Result<()> {
    let file = OpenOptions::new()
        .write(true)
        .read(true)
        .create(true)
        .open("./foo.txt")?;

    write("./foo.txt", "Teste")?;

    let mut buf_reader = BufReader::new(file);
    let mut contents = String::new();

    buf_reader.read_to_string(&mut contents)?;
    println!("{contents}");

    Ok(())
}
