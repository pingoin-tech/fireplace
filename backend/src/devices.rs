use fireplace::devices::Device;
pub mod shellies;

use std::sync::Mutex;

use fireplace::eventhandler::Value;

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
    if let Ok(mut list_option) = SENSOR_LIST.lock() {
        if let Some(list) = list_option.as_mut() {
            match list.into_iter().find(|x| x.id == id) {
                Some(device) => found(device),
                None => not_found(list),
            }
        } else {
            error_val
        }
    } else {
        error_val
    }
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
