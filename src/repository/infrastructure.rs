use serde_derive::{Serialize, Deserialize};

#[derive(Deserialize, Serialize)]
pub struct Infrastructure {
    id: String,
    name: String,
    infra_type: InfraType,
    amount: f64,
    amount_name: String,
    cost: f64,
    building_time: i8,
    effect_time: i8,
    ranking_effects: Vec<RankingEffect>
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
    PoliceStation,
    BurocracyDepartment,
    Airport,
    Port,
    Toll
}

#[derive(Deserialize, Serialize)]
struct RankingEffect {
    index: String,
    score: f64
}

pub fn read_file(input_path: &str) -> Result<Vec<Infrastructure>, std::io::Error> {

    let infrastructure = {
        let text = std::fs::read_to_string(&input_path)?;

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