use crate::repository::file::{read_file, write_file};

mod repository;
fn main() {
    println!("Hello, world!");

    let object = read_file("infra.json");

    write_file(object.unwrap());
    
}
