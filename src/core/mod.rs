use crate::repository::infrastructure::Infrastructures;
use bevy::prelude::*;

pub mod population;

pub fn generate_next_turn() {

    population::generate_population();
    
    population::generate_complainings();
    // TODO call generate_news
    // TODO Check if an infrastructure is completed, and show in the next turn as ready. Add this infrastructure in the ready list, and start to count the effect time.
    // TODO Check if an infrastructure reached the effect time, and change the score.
    // TODO Recalculate the ranking of each index.
    // TODO Calculate the incomes and expenses.

    println!("advance to the next turn")
}

pub fn add_game_infra(infra: Res<Infrastructures>) {
    
    println!("add_game_infra {:?}", infra);
    println!("name: {} ", infra.list.get(0).take().unwrap().name)
}