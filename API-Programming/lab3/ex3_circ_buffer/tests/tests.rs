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

#[test]
pub fn circular_buffer_box() {
}