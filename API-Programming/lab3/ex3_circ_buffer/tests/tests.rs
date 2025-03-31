use circular_buffer::buffer::{CircularBuffer, BufferError};

#[test]
pub fn circular_buffer_insert() {
    let mut cb:CircularBuffer<i32> = CircularBuffer::new(10);
    cb.write(1).unwrap();
    assert_eq!(cb.size(), 10);
    assert_eq!(cb.num(), 1);
}

#[test]
pub fn circular_buffer_write_read() {
    let mut cb:CircularBuffer<i32> = CircularBuffer::new(10);
    cb.write(1).unwrap();
    assert_eq!(cb.read().unwrap(), 1);
}

/*
#[test]
pub fn circular_buffer_box() {
    let mut cb: CircularBuffer<Box<Option<dyn Any>>> = CircularBuffer::new(10);

    cb.write(Box::new(Some(10))).unwrap();
    cb.write(Box::new(Some("ciao".to_string()))).unwrap();

    assert_eq!(cb.num(), 2);

    let v1 = cb.read().unwrap();
    let v2 = cb.read().unwrap();

    assert_eq!(*v1.downcast::<i32>().unwrap(), 10);
    assert_eq!(*v2.downcast::<String>().unwrap(), "ciao".to_string());
}
 */