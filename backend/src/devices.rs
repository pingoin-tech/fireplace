use fireplace::devices::Device;
pub mod shellies;

use std::sync::Mutex;

use fireplace::eventhandler::Value;

use crate::utils::open_locked_mutex_option;

type DeviceDataBase = Mutex<Option<Vec<Device>>>;

pub static SENSOR_LIST: DeviceDataBase = Mutex::new(None);

pub fn init_sensor_list() {
    SENSOR_LIST
        .lock()
        .expect("could not lock")
        .get_or_insert(Vec::new());
}

pub fn get_device_from_list<Fs, Ff, T>(id: String, found: Fs, not_found: Ff, error_val: T) -> T
where
    Fs: FnOnce(&mut Device) -> T,
    Ff: FnOnce(&mut Vec<Device>) -> T,
{
    open_locked_mutex_option(
        &SENSOR_LIST,
        |list| match list.into_iter().find(|x| x.id == id) {
            Some(device) => found(device),
            None => not_found(list),
        },
        error_val,
    )
}

pub fn insert_value_in_device(id: String, key: String, val: Value) -> bool {
    get_device_from_list(
        id,
        |device| {
            device.values.insert(key, val);
            true
        },
        |_| false,
        false,
    )
}
