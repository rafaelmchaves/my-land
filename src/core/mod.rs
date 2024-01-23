pub mod population;

pub fn generate_next_turn(memory_data: MemoryData) {

    population::generate_population(&memory_data);
    
    population::generate_complainings(&memory_data);
    // TODO call generate_news
    // TODO Check if an infrastructure is completed, and show in the next turn as ready. Add this infrastructure in the ready list, and start to count the effect time.
    // TODO Check if an infrastructure reached the effect time, and change the score.
    // TODO Recalculate the ranking of each index.
    // TODO Calculate the incomes and expenses.

    println!("advance to the next turn")
}

pub struct MemoryData {

}