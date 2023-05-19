use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;

#[derive(Deserialize, Serialize, Clone, Debug, PartialEq)]
pub struct ConfigFile {
    #[serde(default)]
    pub extra_links: Vec<Link>,
    pub mqtt_broker: Server,
    pub http_server: Server,
    pub influx_server: Option<Server>,
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
            influx_server: None,
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
    #[serde(default)]
    pub host: String,
    #[serde(default)]
    pub port: u16,
    #[serde(default)]
    pub user: Option<String>,
    #[serde(default)]
    pub password: Option<String>,
}

#[derive(Deserialize, Serialize, Clone, Debug, Default, PartialEq)]
pub struct DeviceSetup {
    #[serde(default)]
    pub alias: String,
    #[serde(default)]
    pub logged_values: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub device_type: Option<String>,
}
