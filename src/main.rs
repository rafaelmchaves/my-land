use crate::repository::{policy, ranking};
use crate::ui::open;
use std::io;

mod repository;
mod ui;
mod core;

fn main() -> io::Result<()> {

    let mut command:String = String::new();
    while command!= "-1".to_string() {
    
        println!("Type a number");
        command = std::io::stdin().lines().next().unwrap().unwrap();
       
        println!("command: {}", &command);
        policy::get_all_policies();
        
        ranking::get_ranking_by_name_and_year("health", "1880");

    // open();
    }
    Ok(())
}
