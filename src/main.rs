use crate::repository::{policy, ranking};
use crate::ui::open;
use std::io;

mod repository;
mod ui;
mod core;

fn main() -> io::Result<()> {

    let mut option:String = String::new();

    println!("--------- Welcome to the game!! ---------");

    policy::get_all_policies();
        
    ranking::get_ranking_by_name_and_year("health", "1880");
    
    while option!= "-1".to_string() {
    
        if option == "1" {
            build_infrastructure_options();
        } 
        if option == "2" {
            build_policy_options();
        } 
        if option == "3" {

        }
        else {
            
            build_initial_menu(&mut option);
        }
        
    // open();
    }
    Ok(())
}

fn build_initial_menu(option: &mut String) {
    println!("Select one of the options below:");
    println!("1 - Build an infrastructure");
    println!("2 - Add a policy");
    println!("3 - Budget");
    println!("4 - Informations about your city");
    *option = std::io::stdin().lines().next().unwrap().unwrap();
}

fn build_infrastructure_options() {
    println!("Select one of the options below:");
    println!("1 - Build a Small hospital");
    println!("2 - Build a Big Hospital");
    println!("3 - Build a School");
    println!("-1 - Return to the previous menu");
    let option = std::io::stdin().lines().next().unwrap().unwrap();

}

fn build_policy_options() {
    println!("Select one of the options below:");
    println!("1 - Add a policy x");
    println!("-1 - Return to the previous menu");
    let option = std::io::stdin().lines().next().unwrap().unwrap();
}
