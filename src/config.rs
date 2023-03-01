use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Clone, Debug, PartialEq)]
pub struct ConfigFile {
    pub extra_links: Vec<Link>,
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
        Self { extra_links: links }
    }
}

#[derive(Deserialize, Serialize, Clone, Debug, PartialEq)]
pub struct Link {
    pub name: String,
    pub address: String,
}
