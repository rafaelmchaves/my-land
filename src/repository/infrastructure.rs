use serde_derive::{Serialize, Deserialize};

#[derive(Deserialize, Serialize)]
pub struct Infrastructure {
    id: String,
    name: String,
    infra_type: InfraType,
    amount: f64,
    amount_name: String,
    cost: f64,
    monthly_cost: f64,
    building_time: u8,
    effect_time: u8,
    ranking_effects: Vec<RankingEffect>,
    description: String,
    max_usage: u32 //max number of people that this infrastructure supports
}

#[derive(Deserialize, Serialize)]
pub enum InfraType {
    Road,
    Street,
    Park,
    Hospital,
    School,
    College,
    University,
    Cemitery,
    FireDepartment,
    Police,
    Justice,
    BureaucracyDepartment,
    Airport,
    Port,
    Toll,
    Entertainment
}

#[derive(Deserialize, Serialize)]
pub struct RankingEffect {
    index: String,
    score: f64
}

pub fn get_all_infrastructures() -> Result<Vec<Infrastructure>, std::io::Error> {

    let infrastructure = {
        let text = std::fs::read_to_string("infra.json")?;

        // Load the InfrastructureFile structure from the string.
        serde_json::from_str::<Vec<Infrastructure>>(&text).unwrap()

    };

    Ok(infrastructure)
    
}

pub fn write_file(infrastructure: Vec<Infrastructure>) -> Result<(), std::io::Error> {
    std::fs::write(
        "file_test.json",
        serde_json::to_string_pretty(&infrastructure).unwrap(),
    )?;

    Ok(())
}