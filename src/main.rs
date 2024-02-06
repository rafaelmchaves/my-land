use repository::infrastructure::Infrastructure;
use crate::repository::{policy, ranking};
use crate::ui::{open, terminal_menu};
use std::io;
use std::collections::HashMap;

mod repository;
mod ui;
mod core;

struct GameData {
     building_list: Vec<Infrastructure>,
     in_construction_buildings: Vec<Infrastructure>,
     week: u8,
     year: u16,
     budget: Budget
}

struct Budget {
    balance: f64,
    expenses: HashMap<String, Expense>,
    incomes: HashMap<String, Income>

}

struct Expense {
    category: String,
    cost: f64
}

struct Income {
    category: String,
    value: f64
}

impl GameData {
    fn new() -> GameData {
        GameData {
            building_list: vec![],
            in_construction_buildings: vec![], 
            week: 1,
            year: 1880,
            budget: Budget {
                balance: 12330.00,
                expenses: HashMap::new() ,
                incomes: HashMap::new()
            }
        }  
    }
}

fn main() -> io::Result<()> {

    let mut game_data = GameData::new();

    println!("--------- Welcome to the game!! ---------");

    policy::get_all_policies();
        
    ranking::get_ranking_by_name_and_year("health", "1880");

    terminal_menu::start(game_data);

    Ok(())
}