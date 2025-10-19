use tracing::{
    info,
    error,
    debug
};

use std::collections::HashMap;

use serde::Deserialize;




pub const DEFAULT_HTTP_PORT: u16 = 80;


#[derive(Debug, Deserialize)]
struct EnvironmentConfig {
    http_port: Option<u16>,
    cn: Option<String>
}


#[derive(Debug, Clone)]
pub struct Config {
    http_port: u16,
    connections: HashMap<String, String>
}



impl Config {
    pub fn from_env() -> Self {
        let cfg: Config = match envy::from_env::<EnvironmentConfig>() {
            Ok(config) => {
                debug!("Configuration: {:?}", config);
                match config.cn {
                    Some(cn) => {
                        let mut connection_strings: HashMap<String, String> = HashMap::new();

                        let kvs: Vec<&str> = cn.split(",").collect();
                        for kv in kvs.iter() {
                            let pair: Vec<&str> = kv.split("=").collect();
                            connection_strings.insert(pair[0].to_string(), pair[1].to_string());
                        }

                        let cfg = Config {
                            http_port: config.http_port.unwrap_or(DEFAULT_HTTP_PORT),
                            connections: connection_strings.clone(),
                        };

                        debug!("cfg: {:?}", cfg);
                        cfg
                    },
                    None => {
                        error!("No CN provided in configuration");
                        Config {
                            http_port: DEFAULT_HTTP_PORT,
                            connections: HashMap::new(),
                        }
                    }
                }
            }
            Err(error) => {
                error!("Failed to load configuration from environment: {:?}", error);
                Config {
                    http_port: DEFAULT_HTTP_PORT,
                    connections: HashMap::new(),
                }
            }
        };

        return cfg;
    }

    pub fn connections(&self) -> HashMap<String, String> {
        return self.connections.clone();
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    // #[test]
    // fn it_works() {
    //     let result = add(2, 2);
    //     assert_eq!(result, 4);
    // }
}
