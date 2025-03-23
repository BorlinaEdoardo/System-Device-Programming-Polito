use std::ops::Add;

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
        Board{
            boats: boats.try_into().unwrap(),
            data: [[0; BSIZE]; BSIZE]
        }
    }

    /* create a new board with a string representing the whole content of board.txt file */
    pub fn from(s: String)->Board {
        let fields = s.split("\n").map(ToString::to_string).collect::<Vec<String>>();
        let boats = fields[0].split(" ").
            map(|c| c.parse::<u8>().unwrap()).
            collect::<Vec<u8>>();
        let mut board = Board::new(&boats);
        let mut i = 0;
        let mut j = 0;
        for row in &fields[1..]{
            for cell in row.split(" "){
                board.data[i][j] = if cell == "B" {1} else {0};
                j += 1;
            }
            i += 1;
        }

        return board;
    }

    /* add a boat to the board if possible, return  */
    pub fn add_boat(&mut self, boat: Boat, pos: (usize, usize)) -> Result<&mut Self, Error> {
        if pos.0 >= BSIZE || pos.1 >= BSIZE {
            return Err(Error::OutOfBounds);
        }

        match boat {
            Boat::Vertical(size) => {
                if self.boats[size-1] == 0{
                    return Err(Error::BoatCount);
                }
                for i in 0..size{
                    if self.data[pos.0+i][pos.1] == 1 {
                        return Err(Error::Overlap);
                    }
                    self.data[pos.0][pos.1] = 1;
                }
            },
            Boat::Horizontal(size) => {
                if self.boats[size-1] == 0{
                    return Err(Error::BoatCount);
                }
                for i in 0..size{
                    if self.data[pos.0][pos.1+i] == 1 {
                        return Err(Error::Overlap);
                    }
                    self.data[pos.0][pos.1] = 1;
                }
            }
        }

        Ok(self)
    }

    fn boat_to_string(boats: &[u8]) -> String{
        return boats.iter().map(|b| (*b).to_string()).collect::<Vec<String>>().join(" ");
    }

    /* converte la board in una stringa salvabile su file */
    pub fn to_string(&self) -> String{
        let mut ret_val = Self::boat_to_string(&self.boats);
        ret_val.push_str("\n");

        for row in &self.data {
            for cell in *row {
                if cell == 0 {
                    ret_val.push_str("- ");
                } else {
                    ret_val.push_str("B ");
                }
            }
            ret_val.push_str("\n");
        }
        return ret_val;
    }
}



fn main() {
    println!("Hello, world!");
}


