use serde::{Deserialize, Serialize};
use ts_rs::TS;
#[derive(Serialize, Deserialize, Debug)]
pub struct ShellyAnnounce {
    pub id: String,
    pub model: String,
    pub mac: String,
    pub ip: String,
    pub new_fw: bool,
    pub fw_ver: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mode: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Default, Clone, TS)]
#[ts(export)]
pub struct WifiState {
    connected: bool,
    ssid: String,
    ip: String,
    pub rssi: i16,
}

#[derive(Serialize, Deserialize, Debug, Default, Clone, TS)]
#[ts(export)]
pub struct RelaysState {
    pub ison: bool,
    pub has_timer: bool,
    pub timer_started: i32,
    pub timer_duration: i32,
    pub timer_remaining: i32,
    pub source: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub overpower: Option<bool>,
}

#[derive(Serialize, Deserialize, Debug, Default, Clone, TS)]
#[ts(export)]
pub struct MeterStat {
    power: f32,
    #[serde(skip_serializing_if = "Option::is_none")]
    overpower: Option<f32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    timestamp: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    counters: Option<Vec<f32>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    total: Option<f32>,
    is_valid: bool,
}

#[derive(Serialize, Deserialize, Debug, Default, Clone, TS)]
#[ts(export)]
pub struct InputStat {
    input: u8,
    event: String,
    event_cnt: u32,
}

#[derive(Serialize, Deserialize, Debug, Default, Clone, TS)]
#[ts(export)]
pub struct UpdateStat {
    status: String,
    has_update: bool,
    new_version: String,
    old_version: String,
}

#[derive(Serialize, Deserialize, Debug, Default, Clone, TS)]
#[ts(export)]
pub struct LightStat {
    pub ison: bool,
    pub source: String,
    pub has_timer: bool,
    pub timer_started: u32,
    pub timer_duration: u32,
    pub timer_remaining: u32,
    pub mode: String,
    pub brightness: u8,
    pub transition: u16,
}
#[derive(Serialize, Deserialize, Debug, Default, Clone, TS)]
#[ts(export)]
pub struct TemperatureStat {
    #[serde(rename = "tC")]
    t_c: f32,
    #[serde(rename = "tF")]
    t_f: f32,
    is_valid: bool,
}

#[derive(Serialize, Deserialize, Debug, Default, Clone, TS)]
#[ts(export)]
pub struct RollerStat {
    state: String,
    source: String,
    power: f32,
    is_valid: bool,
    safety_switch: bool,
    overtemperature: bool,
    stop_reason: String,
    last_direction: String,
    current_pos: u8,
    calibrating: bool,
    positioning: bool,
}

#[derive(Serialize, Deserialize, Debug, Default, Clone, TS)]
#[ts(export)]
pub struct ShellyInfo {
    pub wifi_sta: WifiState,
    time: String,
    unixtime: u64,
    has_update: bool,
    mac: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub relays: Option<Vec<RelaysState>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lights: Option<Vec<LightStat>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rollers: Option<Vec<RollerStat>>,
    pub meters: Vec<MeterStat>,
    pub inputs: Vec<InputStat>,
    pub update: UpdateStat,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tmp: Option<TemperatureStat>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub overtemperature: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub overpower: Option<bool>,
    ram_total: u32,
    ram_free: u32,
    fs_size: u32,
    fs_free: u32,
    pub uptime: u32,
}
