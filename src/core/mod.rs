use crate::{repository::infrastructure::Infrastructures, GameData};
use bevy::prelude::*;

pub mod population;

pub fn generate_next_turn(game_data: &mut GameData) {

    population::generate_population();
    
    population::generate_complainings();
    // TODO call generate_news

    for item in  game_data.in_construction_buildings.iter_mut() {
        item.building_time -= 1;
    }
    // TODO Check if an infrastructure is completed, and show in the next turn as ready. Add this infrastructure in the ready list, and start to count the effect time.
    // TODO Check if an infrastructure reached the effect time, and change the score.
    // TODO Recalculate the ranking of each index.
    // TODO Calculate the incomes and expenses.

    advance_time(game_data);

    println!("advanced to the next turn, week: {}, year: {} ", game_data.week, game_data.year)
}

fn advance_time(game_data: &mut GameData) {
    if game_data.week == 52 {
        game_data.year += 1;
        game_data.week = 1;
    } else {
        game_data.week +=1;
    }
}

pub fn add_game_infra(infra: Res<Infrastructures>) {
    
    println!("add_game_infra {:?}", infra);
    println!("name: {} ", infra.list.get(0).take().unwrap().name)
}