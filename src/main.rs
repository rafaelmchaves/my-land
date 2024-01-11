use crate::repository::infrastructure::{read_file, write_file};
use crate::repository::policy;

mod repository;
fn main() {
    println!("Hello, world!");

    let object = read_file("infra.json");

    write_file(object.unwrap());


    policy::get_all_policies();
    
}
