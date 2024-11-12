use async_openai::config::AzureConfig;

pub struct Settings {
    pub az_openai_config: AzureConfig
}

impl Settings {
    pub fn new() -> Self {
        let openai_api_base = Self::read_env_var("OPENAI_API_BASE", true);
        let openai_api_key = Self::read_env_var("OPENAI_API_KEY", true);
        let openai_api_deployment_id = Self::read_env_var("OPENAI_API_DEPLOYMENT_ID", true);
        let openai_api_version = Self::read_env_var("OPENAI_API_VERSION", true);

        let az_openai_config = AzureConfig::new()
            .with_api_base(openai_api_base)
            .with_api_key(openai_api_key)
            .with_deployment_id(openai_api_deployment_id)
            .with_api_version(openai_api_version);

        Settings {
            az_openai_config
        }
    }

    fn read_env_var(name: &str, mandatory: bool) -> String {
        if mandatory {
            std::env::var(name).expect(&format!("Environment variable {} is not set", name))
        } else {
            std::env::var(name).unwrap_or_default()
        }
    }
}