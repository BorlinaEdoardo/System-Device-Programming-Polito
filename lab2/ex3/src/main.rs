// imports
use std::fs::File;
use std::io::{Read, Write};
use std::time::{SystemTime, UNIX_EPOCH};

// Enums and custom types
struct Node {
    name: String,
    size: u32,
    count: u32,
}
impl Node {
    pub fn new(name: &str) -> Node {
        Node {name: name.to_string(), size: 0, count: 0}
    }

    pub fn size(&mut self, n: u32) -> &mut Self{
        self.size = n;
        return self;
    }
    pub fn count(&mut self, c: u32) -> &mut Self{
        self.count = c;
        return self;
    }

    pub fn to_string(&self) -> String {
        return format!("Name: {}, size: {}, count: {}", self.name, self.size, self.count);
    }

    pub fn grow(&mut self){
        self.size += 1;
    }

    pub fn inc(&mut self){
        self.count += 1;
    }
}


// Enum Error
enum Error{
    Simple(SystemTime),
    Complex(SystemTime, String)
}

#[derive(Debug, PartialEq)]
pub enum MulErr {
    Overflow,
    NegativeNumber
}

fn print_error(e: Error) {
    match e {
        Error::Simple(time) => {
            let duration = time.duration_since(UNIX_EPOCH).unwrap_or_default();
            println!("Error Type: Simple");
            println!("Timestamp: {} seconds since UNIX epoch", duration.as_secs());
        }
        Error::Complex(time, message) => {
            let duration = time.duration_since(UNIX_EPOCH).unwrap_or_default();
            println!("Error Type: Complex");
            println!("Timestamp: {} seconds since UNIX epoch", duration.as_secs());
            println!("Message: {}", message);
        }
    }
}

// base functions
/*
    1.  Aprire, leggere e salvare un file: leggere un file “test.txt” con dentro del testo e
        salvare il testo ripetuto 10 volte nello stesso file
        Usare le funzioni read_to_string e write definite in std::fs
        (https://doc.rust-lang.org/std/fs/)
 */

fn read_file(file_name:&str) -> Result<(), Box<dyn std::error::Error>>{
    let mut file = File::open(file_name)?;
    let mut out_file = File::create("out.txt")?;

    let mut content = String::new();
    let _ = file.read_to_string(&mut content);

    for i in 0..=9 {
        out_file.write_all(content.as_bytes())?;
    }

    out_file.sync_all()?;
    file.sync_all()?;
    Ok(())
}

pub fn mul(a: i32, b: i32) -> Result<u32, MulErr> {
    if (a < 0) ^ (b < 0) {  // XOR: true if only one is negative
        return Err(MulErr::NegativeNumber);
    }

    match a.checked_mul(b) {
        Some(res) if res >= 0 => Ok(res as u32),
        _ => Err(MulErr::Overflow),
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_node_creation() {
        let node = Node::new("test_node");
        assert_eq!(node.name, "test_node");
        assert_eq!(node.size, 0);
        assert_eq!(node.count, 0);
    }

    #[test]
    fn test_node_methods() {
        let mut node = Node::new("node1");
        node.size(10).count(5);
        assert_eq!(node.size, 10);
        assert_eq!(node.count, 5);

        node.grow();
        node.inc();
        assert_eq!(node.size, 11);
        assert_eq!(node.count, 6);
    }

    #[test]
    fn test_node_to_string() {
        let mut node = Node::new("test");
        node.size(3).count(7);
        assert_eq!(node.to_string(), "Name: test, size: 3, count: 7");
    }

    #[test]
    fn test_mul_success() {
        assert_eq!(mul(5, 4), Ok(20));
        assert_eq!(mul(0, 100), Ok(0));
    }

    #[test]
    fn test_mul_negative_number() {
        assert_eq!(mul(-5, 3), Err(MulErr::NegativeNumber));
        assert_eq!(mul(5, -3), Err(MulErr::NegativeNumber));
    }

    #[test]
    fn test_mul_overflow() {
        assert_eq!(mul(i32::MAX, 2), Err(MulErr::Overflow));
    }

    #[test]
    fn test_read_file_nonexistent() {
        let result = read_file("nonexistent.txt");
        assert!(result.is_err());
    }
}

fn main() {
    read_file("tests.txt");

    let now = SystemTime::now();

    let err1 = Error::Simple(now);
    let err2 = Error::Complex(now, "File not found".to_string());

    print_error(err1);
    println!("------------------");
    print_error(err2);

}
