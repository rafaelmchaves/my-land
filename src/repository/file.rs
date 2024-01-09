use serde_derive::{Serialize, Deserialize};


#[derive(Deserialize, Serialize, Debug)]
pub struct InfrastructureFile {
    id: String,
    name: String,
    cost: f64
}

pub fn read_file(input_path: &str) -> Result<InfrastructureFile, std::io::Error> {

    let infrastructure = {
        let text = std::fs::read_to_string(&input_path)?;

        // Load the InfrastructureFile structure from the string.
        serde_json::from_str::<InfrastructureFile>(&text).unwrap()

    };

    Ok(infrastructure)
    
}

pub fn write_file(infrastructure: InfrastructureFile) -> Result<(), std::io::Error> {
    std::fs::write(
        "file_test.json",
        serde_json::to_string_pretty(&infrastructure).unwrap(),
    )?;

    Ok(())
}