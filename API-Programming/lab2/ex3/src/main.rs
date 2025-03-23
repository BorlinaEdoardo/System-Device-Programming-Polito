const BSIZE: usize = 20;
pub struct Board {
    boats: [u8; 4],
    data: [[u8; BSIZE]; BSIZE],
}
pub enum Error {
    Overlap,
    OutOfBounds,
    BoatCount,
}
pub enum Boat {
    Vertical(usize),
    Horizontal(usize)
}
impl Board {
    /** Create a new board with the respective available ships */
    pub fn new(boats: &[u8]) -> Board {

    }

    /* create a new board with a string representing the whole content of board.txt file */
    pub fn from(s: String)->Board {

    }

    /* add a boat to the board if possible, return  */
    pub fn add_boat(&mut self, boat: Boat, pos: (usize, usize)) -> Result<&mut Self, Error> {

    }

    /* converte la board in una stringa salvabile su file */
    pub fn to_string(&self) -> String
}



fn main() {
    println!("Hello, world!");
}


