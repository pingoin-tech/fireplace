use std::collections::BTreeMap;

use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Clone, Debug, PartialEq)]
pub struct ConfigFile {
    pub extra_links: Vec<Link>,
    pub mqtt_broker: Server,
    pub http_server: Server,
    pub device_settings: BTreeMap<String, DeviceSetup>,
}

impl Default for ConfigFile {
    fn default() -> Self {
        let links = vec![
            Link {
                name: "Awsome Project".to_string(),
                address: "https://github.com/pingoin-tech/fireplace".to_string(),
            },
            Link {
                name: "Awsome Language".to_string(),
                address: "https://www.rust-lang.org/".to_string(),
            },
        ];
        let mqtt = Server {
            host: "localhost".to_string(),
            port: 1883,
            user: None,
            password: None,
        };
        let http = Server {
            host: "0.0.0.0".to_string(),
            port: 8080,
            user: None,
            password: None,
        };
        Self {
            extra_links: links,
            mqtt_broker: mqtt,
            http_server: http,
            device_settings: BTreeMap::new(),
        }
    }
}

#[derive(Deserialize, Serialize, Clone, Debug, PartialEq)]
pub struct Link {
    pub name: String,
    pub address: String,
}

#[derive(Deserialize, Serialize, Clone, Debug, Default, PartialEq)]
pub struct Server {
    pub host: String,
    pub port: u16,
    pub user: Option<String>,
    pub password: Option<String>,
}

#[derive(Deserialize, Serialize, Clone, Debug, Default, PartialEq)]
pub struct DeviceSetup {
    pub alias: String,
}
