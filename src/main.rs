use repository::infrastructure::Infrastructure;

use crate::repository::{policy, ranking, infrastructure};
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
    
        match option.as_str() {
             "1" => build_infrastructure_options(&mut option),
             "2" => build_policy_options(&mut option),
               _ => build_initial_menu(&mut option),
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

fn build_infrastructure_options(option: &mut String) {

    let infra_result = infrastructure::get_all_infrastructures();
    if infra_result.is_ok() {
        let infra_list = infra_result.unwrap();
  
        while option.as_str() != "-1" && option.as_str() != "y" {
            println!("Select one of the options below:");
            for item in &infra_list {
                println!("{} - Build {}", item.id, item.name);
            }
            
            println!("-1 - Return to the previous menu");
            *option = std::io::stdin().lines().next().unwrap().unwrap();
            if option.as_str() != "-1" {

                let infra_selected = select_infrastructure(&infra_list, option);
                if (&infra_selected).is_some() {
                
                    let item = infra_selected.unwrap();
                    let mut option_confirmation:String = String::new();
                    create_confirmation_selected(&item, &mut option_confirmation);
                    if option_confirmation.as_str() == "y" {
                        *option = "y".to_string();
                        println!("we will start to build the {}", item.name)
                    }
                    
                } else {
                    println!("Select a valid option");
                }
                
            } 
        }
        
    } else {
        println!("It was not found any infrastructure. Try again");
    }

    *option = "0".to_string();

}

fn create_confirmation_selected(item: &Infrastructure, option: &mut String) {
    println!("-----------------------------------------");
    println!("The element that you want to build is: ");
    println!("Name: {}", item.name);
    println!("Price: ${}", item.cost);
    println!("Time to build: {} weeks", item.building_time);
    println!("Time to take effect after built: {} weeks", item.effect_time);
    println!("Monthly cost: ${} ", item.monthly_cost);

    println!("Do you want to build? y/n ");
    *option = std::io::stdin().lines().next().unwrap().unwrap();
}

fn select_infrastructure(infra_list: &Vec<Infrastructure>, option: &mut String) -> Option<Infrastructure> {
    for item in infra_list {
        if item.id == option.to_string() {
            return Some(item.clone())
        }
    }
    None
}

fn build_policy_options(option: &mut String) {
    println!("Select one of the options below:");
    println!("1 - Add a policy x");
    println!("-1 - Return to the previous menu");
    *option = std::io::stdin().lines().next().unwrap().unwrap();
    match option.as_str() {
        "1" => build_policy_x(),
          _ => println!("returning to menu")
    }

    *option = "0".to_string();
  
}

fn build_policy_x() {
    println!("");
    println!("build_policy_x");
    println!("");
}
