pub trait Sensor {
   fn get_actions(&self)->Vec<EventAction>;
   fn get_events(&self)->Vec<EventAction>;
}

pub struct EventAction{
    name:String,
    parameter:ParamType,
}

pub enum ParamType{
    NoParam,
    StringParam,
    FloatParam,
    IntegerParam,
}