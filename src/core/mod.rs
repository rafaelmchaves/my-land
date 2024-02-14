use crate::{repository::{infrastructure::Infrastructures, ranking}, GameData};
use bevy::prelude::*;

pub mod population;

pub fn generate_next_turn(game_data: &mut GameData) {

    population::generate_population();
    
    population::generate_complainings();
    // TODO call generate_news

    //reduce the remaining time to build a infrastructure and when the bulding is ready, add to the building list
    for i in 0..game_data.in_construction_buildings.len() {
        let item = game_data.in_construction_buildings.get_mut(i).unwrap();
        item.building_time -= 1;
        if item.building_time == 0 {
            game_data.building_list.push(item.clone());
            game_data.in_construction_buildings.swap_remove(i);
        }
    
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
        ranking::get_ranking_by_name_and_year("ranking/health", game_data.year.to_string().as_str()).unwrap();
        //TODO load the new ranking of countries for each index in that new year
    } else {
        game_data.week +=1;
    }
}

pub fn add_game_infra(infra: Res<Infrastructures>) {
    
    println!("add_game_infra {:?}", infra);
    println!("name: {} ", infra.list.get(0).take().unwrap().name)
}