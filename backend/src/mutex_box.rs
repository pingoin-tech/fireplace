use std::sync::Mutex;

pub struct MutexBox<T> {
    pub name: &'static str,
    pub mutex: Mutex<Option<T>>,
}

impl<T> MutexBox<T> {
    pub const fn new(name: &'static str) -> Self {
        let mutex: Mutex<Option<T>> = Mutex::new(None);
        MutexBox {
            name: name,
            mutex: mutex,
        }
    }

    pub fn open_locked<FunctionLocked, TypeReturn>(
        &self,
        found: FunctionLocked,
        error_val: TypeReturn,
    ) -> TypeReturn
    where
        FunctionLocked: FnOnce(&mut T) -> TypeReturn,
    {
        if let Ok(mut handler_option) = self.mutex.lock() {
            if let Some(handler) = handler_option.as_mut() {
                found(handler)
            } else {
                error_val
            }
        } else {
            error_val
        }
    }

    pub fn init(&self, data: T) {
        self.mutex
            .lock()
            .expect("could not lock")
            .get_or_insert(data);
    }
}
