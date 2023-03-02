use serde::{Deserialize, Serialize};
use std::error::Error;
use std::fs::{write, OpenOptions};
use std::io::{prelude::*, BufReader, Result};

#[derive(Debug, Serialize, Deserialize)]
struct Student {
    name: String,
    last_name: String,
    age: u16,
    cpf: String,
}

impl Student {
    fn new(name: String, last_name: String, age: u16, cpf: String) -> Student {
        Student {
            name,
            last_name,
            age,
            cpf,
        }
    }
}

fn main() -> Result<()> {
    let file = OpenOptions::new()
        .write(true)
        .read(true)
        .create(true)
        .open("./foo.txt")
        .expect("cant open this file...");

    let new_student = Student::new(
        String::from("Gabriel"),
        String::from("da Cunha"),
        22,
        String::from("11111111111"),
    );

    write("./foo.txt", serde_json::to_string(&new_student).unwrap())?;
    println!("Succes writing the student \n {:#?}", &new_student);

    let mut buf_reader = BufReader::new(file);
    let mut contents = String::new();

    buf_reader.read_to_string(&mut contents)?;
    //println!("{contents}");

    Ok(())
}
