use fireplace::devices::Device;
pub mod shellies;

use fireplace::eventhandler::Value;

use crate::mutex_box::MutexBox;

pub static SENSOR_LIST: MutexBox<Vec<Device>> = MutexBox::new("SensorList");

pub fn init_sensor_list() {
    SENSOR_LIST.init(Vec::new());
}

pub fn get_device_from_list<Fs, Ff, T>(id: String, found: Fs, not_found: Ff, error_val: T) -> T
where
    Fs: FnOnce(&mut Device) -> T,
    Ff: FnOnce(&mut Vec<Device>) -> T,
{
    SENSOR_LIST.open_locked(
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
