use crate::repository::{policy, ranking};
use crate::ui::open;

mod repository;
mod ui;
mod core;

fn main() {

    policy::get_all_policies();
    
    ranking::get_ranking_by_name_and_year("health", "1880");

    open();
}
