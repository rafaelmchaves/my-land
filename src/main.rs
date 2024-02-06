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

    terminal_menu::start(GameData::new());

    Ok(())
}