use crate::repository::{policy, ranking, infrastructure};
use crate::ui::open;

mod repository;
mod ui;
mod core;

fn main() {
    println!("Hello, world!");

    let object = infrastructure::get_all_infrastructures();

    // infrastructure::write_file(object.unwrap());

    policy::get_all_policies();
    
    ranking::get_ranking_by_name_and_year("health", "1880");

    open();
}
