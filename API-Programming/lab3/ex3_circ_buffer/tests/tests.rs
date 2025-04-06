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
pub fn circular_buffer_write_multiple() {
    let mut cb:CircularBuffer<i32> = CircularBuffer::new(10);
    for i in 0..10 {
        cb.write(i).unwrap();
    }
    assert_eq!(cb.num(), 10);

    for i in 0..10 {
        assert_eq!(cb.read().unwrap(), i);
    }
}

#[test]
pub fn circular_buffer_read_when_empty(){
    let mut cb:CircularBuffer<i32> = CircularBuffer::new(10);
    assert_eq!(cb.read(), None);
}

#[test]
pub fn circular_buffer_write_when_full(){
    let mut cb:CircularBuffer<i32> = CircularBuffer::new(10);
    for i in 0..10 {
        cb.write(i).unwrap();
    }
    assert_eq!(cb.write(10).err().unwrap(), BufferError::BufferOverflow);
}

#[test]
pub fn circular_buffer_make_contiguous(){
    let mut cb:CircularBuffer<i32> = CircularBuffer::new(5);
    for _i in 0..5 {
        cb.write(1).unwrap();
    }
    cb.read().unwrap();
    cb.read().unwrap();
    cb.read().unwrap();
    cb.write(1).unwrap();
    cb.write(1).unwrap();
    assert_eq!(cb.to_string(), "1, 1, , 1, 1, ");

    cb.make_contiguous();

    assert_eq!(cb.to_string(), "1, 1, 1, 1, , ");
}

#[test]
pub fn circular_buffer_test_index(){
    let mut cb:CircularBuffer<i32> = CircularBuffer::new(5);
    for i in 0..5 {
        cb.write(i).unwrap();
    }
    assert_eq!(cb[0].unwrap(), 0);
    assert_eq!(cb[4].unwrap(), 4);
}

#[test]
pub fn circular_buffer_test_index_non_contiguous(){
    let mut cb:CircularBuffer<i32> = CircularBuffer::new(5);
    for _i in 0..5 {
        cb.write(1).unwrap();
    }
    cb.read().unwrap();
    cb.read().unwrap();
    cb.read().unwrap();
    cb.write(2).unwrap();
    cb.write(3).unwrap();

    assert_eq!(cb[0].unwrap(), 1);
    assert_eq!(cb[3].unwrap(), 3);
}

#[test]
pub fn circular_buffer_test_mut_index(){
    let mut cb:CircularBuffer<i32> = CircularBuffer::new(5);
    for i in 0..5 {
        cb.write(i).unwrap();
    }
    assert_eq!(cb[2].unwrap(), 2);

    cb[2] = Some(5);

    assert_eq!(cb[2].unwrap(), 5);
}