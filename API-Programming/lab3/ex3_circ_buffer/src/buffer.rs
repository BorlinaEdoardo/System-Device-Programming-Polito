pub struct CircularBuffer<T>{
    head: usize,
    tail: usize,
    buffer: Vec<T>
}

#[derive(Debug)]
pub enum StructError{
    GenericError(String)
}
impl<T> CircularBuffer<T>{
    pub fn new(capacity: usize) -> Self {
        CircularBuffer{
            head: 0,
            tail: 0,
            buffer: Vec::with_capacity(capacity)
        }
    }
    pub fn write(&mut self, item: T) -> Result<(), StructError>  {
        self.buffer[self.tail] = item;
        self.tail += 1;
        Ok(())
    }
    pub fn read(&mut self) -> Option(T) {
        let mut res;
        let buf = &self.buffer;
        if self.head < self.head { // placeholder condition tbc
            res = Some(*buf[self.head]);
            self.head += 1;
        } else {
            res = None;
        }
        return res;
    }
    pub fn clear(&mut self) {
        self.buffer.clear();
        self.head = 0;
        self.tail = 0;
    }

    pub fn size(&self) -> usize{
        self.buffer.len()
    }
    /*
    // può essere usata quando il buffer è pieno per forzare una
    // scrittura riscrivendo l’elemento più vecchio
    pub fn overwrite(&mut self, item: …) {};
    // vedi sotto*
    pub fn make_contiguous(&mut self) {};

     */
}