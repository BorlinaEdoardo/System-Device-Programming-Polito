use std::fs::File;
use std::sync::{Arc, Condvar, Mutex};

struct MyChannel<T>{
    buf: Vec<T>,
    max_size: usize,
    size: Arc<(Mutex<usize>, Condvar)>,
    closed: Arc<Mutex<bool>>,
}

enum Item<T> {Value(T), Stop}

pub enum MyChannelError{
    GenericError(String)
}



impl<T> MyChannel<T> {
    pub fn new(max_size: usize) -> MyChannel<T>{
        MyChannel{
            buf: Vec::with_capacity(max_size),
            max_size,
            size: Arc::new((Mutex::new(0), Condvar::new())),
            closed: Arc::new(Mutex::new(false)),
        }
    }

    pub fn write(item: T) -> Result<(),MyChannelError> {

    }
    pub fn read() -> Result<T, MyChannelError> {todo!()}
    pub fn close() {}
}

fn main() {

}
