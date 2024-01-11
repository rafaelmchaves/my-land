use serde_derive::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
pub struct Ranking {
    list: Vec<CountryPosition>
}

#[derive(Deserialize, Serialize)]
pub struct CountryPosition {
    name: String,
    score: f64,
    position: u8
}

pub fn get_ranking_by_name_and_year(ranking_name: &str, year: &str) -> Result<Ranking, std::io::Error> {

    let countries = {

        let file_path = format!("ranking/{}/{}.json", ranking_name, year);
        let text = std::fs::read_to_string(file_path)?;

        // Load the ranking file structure from the string.
        serde_json::from_str::<Vec<CountryPosition>>(&text).unwrap()

    };

    Ok(Ranking { list: (countries) })
    
}