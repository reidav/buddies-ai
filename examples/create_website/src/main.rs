extern crate buddies_ai;

use dotenv::dotenv;
use std::{collections::HashMap, fs};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv().ok();

    let file = fs::read_to_string("buddies.yaml")?;

    let mut inputs = HashMap::new();
    inputs.insert("city".to_string(), "Paris".to_string());

    let create_website_job = buddies_ai::Job::new(&file, Some(inputs)).unwrap();
    let output = create_website_job.execute().await;
    match output {
        Ok(output) => {
            println!("Output: {}", output);
            Ok(())
        }
        Err(e) => {
            println!("Error: {}", e);
            Err(e)
        }
    }
}