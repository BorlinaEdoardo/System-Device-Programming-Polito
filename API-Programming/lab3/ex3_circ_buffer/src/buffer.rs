use std::fmt;
use std::fmt::{Display, Formatter};
use std::ops::{Index, IndexMut};

pub struct CircularBuffer<T> {
    head: usize,
    tail: usize,
    buffer: Vec<Option<T>>,
    num: usize
}

impl<T: Clone> IntoIterator for CircularBuffer<T> {
    type Item = Option<T>;
    type IntoIter = std::vec::IntoIter<Self::Item>;

    fn into_iter(self) -> Self::IntoIter {
        self.buffer[self.head..self.tail].to_vec().into_iter()
    }
}

#[derive(Debug, Eq, PartialEq)]
pub enum BufferError {
    GenericError(String),
    BufferIsEmpty,
    BufferOverflow
}
impl<T: Clone> CircularBuffer<T>{
    pub fn new(capacity: usize) -> Self {
        CircularBuffer{
            head: 0,
            tail: 0,
            buffer: vec![None; capacity],
            num: 0
        }
    }
    pub fn write(&mut self, item: T) -> Result<(), BufferError> {
        // if buffer is full error is returned
        if self.size() == 0 {
            Err(BufferError::BufferIsEmpty)
        } else if self.num != 0 && self.size() == self.num {
            Err(BufferError::BufferOverflow)
        } else {
            self.buffer[self.tail] = Some(item);
            self.tail = (self.tail + 1) % self.size();
            self.num += 1;
            Ok(())
        }
    }
    pub fn read(&mut self) -> Option<T> {
        let res;
        let mut buf = self.buffer.clone();
        if self.num != 0 {
            res = buf[self.head].take();
            self.head = (self.head + 1) % self.size();
            self.num -= 1;
        } else {
            res = None;
        }
        self.buffer = buf;
        res
    }
    pub fn clear(&mut self) {
        self.buffer.clear();
        self.head = 0;
        self.tail = 0;
        self.num = 0;
    }

    pub fn size(&self) -> usize{
        self.buffer.len()
    }

    pub fn num(&self) -> usize {
        self.num
    }

    // può essere usata quando il buffer è pieno per forzare una
    // scrittura riscrivendo l’elemento più vecchio
    pub fn overwrite(&mut self, item: T) {
        if self.head != self.tail {
            self.write(item).unwrap();
        } else{
            self.buffer[self.tail] = Some(item);
        }
    }
    // make the buffer contiguous
    pub fn make_contiguous(&mut self) {
        let mut v = vec![None; self.size()];
        for i in 0..self.num {
            v[i] = self.buffer[self.head].clone();
            self.head = (self.head + 1) % self.size();
        }
        self.head = 0;
        self.tail = self.num;
        self.buffer = v;
    }
}

impl<T: Display> Display for CircularBuffer<T>{
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        let mut ret_val:fmt::Result = write!(f, "");
        self.buffer.iter().for_each(|item|{
            match item {
                Some(x)=> ret_val = write!(f,"{}, ",x),
                None=> ret_val = write!(f,", ")
            }
        });

        ret_val
    }
}

impl<T: Clone> Index<usize> for CircularBuffer<T> {
    type Output = Option<T>;
    fn index(&self, index: usize) -> &Self::Output {
        let i:usize = (index+self.head)%self.buffer.len();
        if i > self.num {
            panic!();
        }
        &(self.buffer[i])
    }
}

impl<T: Clone> IndexMut<usize> for CircularBuffer<T> {
    fn index_mut(&mut self, index:usize) -> &mut Self::Output {
        let i:usize = (index+self.head)%self.buffer.len();

        if i > self.num {
            panic!();
        }
        &mut self.buffer[i]
    }
}
