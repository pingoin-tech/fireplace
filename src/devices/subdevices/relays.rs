use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Clone, Default, Debug, PartialEq)]
pub struct Relays{
    adress:String,
    on:String,
    off:String,
    toggle:Option<String>,
    state:bool
}
