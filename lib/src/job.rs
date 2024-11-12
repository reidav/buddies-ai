
use std::{collections::HashMap, error::Error};

use yaml_rust::YamlLoader;

use async_openai::{
    config::AzureConfig,
    types::{
        ChatCompletionRequestSystemMessageArgs, ChatCompletionRequestUserMessageArgs,
        CreateChatCompletionRequestArgs
    },
    Client,
};

use crate::settings::Settings;

#[derive(Debug)]
pub struct Buddy {
    pub id: String,
    pub role: String,
    pub backstory: String,
}

#[derive(Debug)]
pub struct Task {
    pub id: String,
    pub description: String,
    pub buddy: String,
    pub expected_output: String,
}

#[derive(Debug)]
pub struct Job {
    az_openai_client: Client<AzureConfig>,
    inputs: Option<HashMap<String, String>>,
    buddies: Vec<Buddy>,
    tasks: Vec<Task>,
}

impl Job {
    pub fn new(job_manifest_as_yaml: &str, inputs : Option<HashMap<String, String>>) -> Result<Job, Box<dyn Error>> {
        // Initialize context
        let settings = Settings::new();
        let az_openai_client = Client::with_config(settings.az_openai_config);

        // Parse the job manifest
        let (buddies, tasks) = Job::from_manifest(job_manifest_as_yaml)?;
        
        Ok(Job {
            inputs,
            az_openai_client,
            buddies,
            tasks,
        })
    }

    fn from_manifest(job_manifest_as_yaml: &str) -> Result<(Vec<Buddy>, Vec<Task>), Box<dyn Error>> {
        let mut buddies: Vec<Buddy> = vec![];
        let mut tasks: Vec<Task> = vec![];

        let docs = YamlLoader::load_from_str(job_manifest_as_yaml).unwrap();
        let doc = &docs[0];

        for buddy in doc["buddies"].as_vec().unwrap() {
            buddies.push(Buddy {
                id: buddy["id"].as_str().unwrap().to_string(),
                role: buddy["role"].as_str().unwrap().to_string(),
                backstory: buddy["backstory"].as_str().unwrap().to_string(),
            });
        }

        for task in doc["tasks"].as_vec().unwrap() {
            tasks.push(Task {
                id: task["id"].as_str().unwrap().to_string(),
                description: task["description"].as_str().unwrap().to_string(),
                buddy: task["buddy"].as_str().unwrap().to_string(),
                expected_output: task["expected_output"].as_str().unwrap().to_string(),
            });
        }

        Ok((buddies, tasks))
    }

    pub async fn execute(&self) -> Result<String, Box<dyn Error>> {

        let mut memories: HashMap<String, String> = HashMap::new();
        println!("Executing the job using sequence of tasks ...");

        for task in &self.tasks {
            let buddy: &Buddy = self.buddies.iter().find(|b| b.id == task.buddy).unwrap();
            
            // Replace prompt with inputs
            let mut task_with_inputs = task.description.clone();
            if let Some(inputs) = &self.inputs {
                for (input_key, input_value) in inputs {
                    println!("Replacing {} with {}", input_key, input_value);
                    task_with_inputs = task_with_inputs.replace(&format!("{{{}}}", input_key), input_value);
                }
            }

            // Handle previous task output from memory into prompt
            for (memory_key, memory_value) in &memories {
                task_with_inputs = task_with_inputs.replace(&format!("{{{}_output}}", memory_key), &memory_value);
            }

            println!("Task: {}", task_with_inputs);
            // Execute the task
            let request = CreateChatCompletionRequestArgs::default()
                .max_tokens(512u32)
                .messages([
                    ChatCompletionRequestSystemMessageArgs::default()
                        .content(vec![buddy.role.as_str(),buddy.backstory.as_str(), &format!("Expected outputs : {}", task.expected_output)].join("."))
                        .build()?
                        .into(),
                    ChatCompletionRequestUserMessageArgs::default()
                        .content(task_with_inputs.as_str())
                        .build()?
                        .into(),
                ])
                .build()?;

            let response = self.az_openai_client.chat().create(request).await?;
            
            // Get results and save to memories
            println!("\nResponse:\n");
            let mut output = String::new();
            for choice in response.choices {
                if let Some(content) = choice.message.content {
                    output.push_str(content.as_str());
                }
            }
            println!("{}", output);

            memories.insert(task.id.clone(), output);
        }

        if let Some(last_memory) = memories.values().last() {
            Ok(last_memory.to_string())
        } else {
            Err("No response".into())
        }
    }

}