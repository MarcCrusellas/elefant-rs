use std::sync::{Arc, Mutex};

pub type Wrapped<T> = Arc<Mutex<T>>;

pub struct Utils;

impl Utils {
    pub fn get_wrapped<T>(t: T) -> Wrapped<T> {
        Arc::new(Mutex::new(t))
    }
}
