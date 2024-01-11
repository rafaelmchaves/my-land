use serde_derive::{Deserialize, Serialize};

use super::infrastructure::RankingEffect;

#[derive(Deserialize, Serialize)]
pub struct Policy {
    title: String,
    description: String,
    cost_for_apply: f64,
    citizen_cost: f64,
    ranking_effects: Vec<RankingEffect>
}

pub fn get_all_policies() -> Result<Vec<Policy>, std::io::Error> {

    let policy = {
        let text = std::fs::read_to_string("policies.json")?;

        // Load the InfrastructureFile structure from the string.
        serde_json::from_str::<Vec<Policy>>(&text).unwrap()

    };

    Ok(policy)
    
}