use std::sync::Mutex;

pub fn open_locked_mutex_option<Fs, Tr, Tm>(
    mutex_option: &Mutex<Option<Tm>>,
    found: Fs,
    error_val: Tr,
) -> Tr
where
    Fs: FnOnce(&mut Tm) -> Tr,
{
    if let Ok(mut handler_option) = mutex_option.lock() {
        if let Some(handler) = handler_option.as_mut() {
            found(handler)
        } else {
            error_val
        }
    } else {
        error_val
    }
}
