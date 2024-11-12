# Buddies_AI

**Buddies_AI** is a lightweight, flexible, and **educational** Rust-based multi-agent framework designed for building coordinated, dynamic systems.

## 📦 Installation

Add Buddies_AI to your project by including it in your `Cargo.toml`:

```toml
[dependencies]
buddies_ai = "0.1.0"
```

##  🚀 Usage
Here's an example of how to use Buddies_AI to create a job that builds and reviews a website:

1. Create a buddies.yaml file with the following plan (automatic plan definition is WIP):

```yaml
buddies:
  - id: dev_frontend_specialist 
    role: >
      Build websites using HTML, CSS, and JavaScript.
    backstory: >
      You've been building websites for a few years now and you're really good at it.
  - id: techlead
    role: >
      Lead a team of developers to build a new product.
    backstory: >
      You have attention to details, a great mentor and assist developers in their growth.
      You provide meaningful feedback to any contribution from the team.
tasks:
  - id: build_website
    description: >
      Develop a single and modern webpage introducing {city}.
    buddy: dev_frontend_specialist
    expected_output: >
      You should only reply using the source code meaning a single webpage built with HTML, CSS, and JavaScript.
  - id: review_website
    description: >
      Evaluate the website source code built by the frontend specialist.
      Eventually provide feedback to improve the website.
      Source code:
      {build_website_output}
    expected_output: >
      Source code reviewed
    buddy: techlead
```

2. Execute the plan :

```rust
extern crate buddies_ai;

use dotenv::dotenv;
use std::{collections::HashMap, fs};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv().ok();

    let file: String = fs::read_to_string("buddies.yaml").expect("File not found");
    let create_website_job = buddies_ai::Job::new(&file);

    let mut inputs = HashMap::new();
    inputs.insert("city".to_string(), "Paris".to_string());

    let _ = match create_website_job.unwrap().execute(Some(inputs)).await {
        Ok(_) => "Job executed successfully",
        Err(error) => panic!("Problem opening the file: {error:?}"),
    };

    Ok(())
}
```
