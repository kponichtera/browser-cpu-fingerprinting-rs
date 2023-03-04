use std::path::Path;
use config::Config;
use log::info;
use serde::Deserialize;

const CONFIG_ENV_PREFIX: &str = "API_SERVER";
const CONFIG_FILE_PATH_DEFAULT: &str = "config/backend";

#[allow(unused)]
#[derive(Clone, Debug, Deserialize)]
pub struct ApiServerConfig {
    pub bind_address: String,
    pub port: u16,
    pub database_connection_count: u32,
    pub database_url: String
}

pub fn read_config(custom_file_path: Option<&Path>) -> ApiServerConfig {
    let mut config_builder = Config::builder();

    match custom_file_path {
        Some(custom_file_path) => {
            info!("Using custom configuration path {}", custom_file_path.to_str().unwrap_or(""));
            config_builder = config_builder.add_source(config::File::from(custom_file_path));
        }
        None => {
            info!("Using default configuration path");
            config_builder = config_builder.add_source(config::File::with_name(CONFIG_FILE_PATH_DEFAULT));
        }
    }

    let config = config_builder
        .add_source(config::Environment::with_prefix(CONFIG_ENV_PREFIX))
        .build()
        .unwrap();

    config.try_deserialize().unwrap()
}

/*
 * TESTS
 */

#[cfg(test)]
mod tests {
    use std::io::Write;
    use crate::config::read_config;

    #[test]
    fn development_config_parses() {
        // execute
        let config = read_config(None);

        // assert
        assert_eq!(config.bind_address, "127.0.0.1")
    }

    #[test]
    #[should_panic]
    fn malformed_config_panics() {
        // prepare
        let config = r#"
            bind_address: 1234
            port: "0.0.0.0"
        "#;

        let mut config_file = tempfile::Builder::new()
            .suffix(".yml")
            .tempfile()
            .unwrap();

        config_file.write_all(config.as_ref()).unwrap();

        // execute (should panic)
        read_config(Some(config_file.path()));
    }
}
