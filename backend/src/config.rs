use config::Config;
use log::info;
use serde::Deserialize;
use std::path::Path;

const CONFIG_ENV_PREFIX: &str = "BACKEND";
const CONFIG_FILE_PATH_DEFAULT: &str = "config/backend";

#[allow(unused)]
#[derive(Clone, Debug, Deserialize)]
pub struct BackendConfig {
    pub bind_address: String,
    pub port: u16,
    pub database_connection_count: u32,
    pub database_url: String,
}

pub fn read_config(custom_file_path: Option<&Path>) -> BackendConfig {
    let mut config_builder = Config::builder();

    match custom_file_path {
        Some(custom_file_path) => {
            info!(
                "Using custom configuration path {}",
                custom_file_path.to_str().unwrap_or("")
            );
            config_builder = config_builder.add_source(config::File::from(custom_file_path));
        }
        None => {
            let default_path = Path::new(CONFIG_FILE_PATH_DEFAULT);

            if default_path.exists() {
                info!("Using default configuration path");
                config_builder =
                    config_builder.add_source(config::File::from(default_path));
            } else {
                info!("No default configuration file detected - using only environment variables");
            }

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
    use crate::config::read_config;
    use std::io::Write;

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

        let mut config_file = tempfile::Builder::new().suffix(".yml").tempfile().unwrap();

        config_file.write_all(config.as_ref()).unwrap();

        // execute (should panic)
        read_config(Some(config_file.path()));
    }
}
