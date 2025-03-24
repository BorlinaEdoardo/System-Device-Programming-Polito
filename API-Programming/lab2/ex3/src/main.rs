use std::ops::Add;

const BSIZE: usize = 20;
pub struct Board {
    boats: [u8; 4],
    data: [[u8; BSIZE]; BSIZE],
}

#[derive(Debug)]
pub enum Error {
    Overlap,
    OutOfBounds,
    BoatCount,
}
pub enum Boat {
    Vertical(usize),
    Horizontal(usize)
}
impl Boat{
    pub fn get_size(&self) -> usize{
        match self {
            Boat::Vertical(size) => *size,
            Boat::Horizontal(size) => *size
        }
    }
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
                if size + pos.0 > BSIZE {
                    return Err(Error::OutOfBounds);
                }else if self.boats[size-1] == 0{
                    return Err(Error::BoatCount);
                }
                for i in 0..size{
                    if self.data[pos.0+i][pos.1] == 1 {
                        return Err(Error::Overlap);
                    }
                    self.data[pos.0+i][pos.1] = 1;
                }
            },
            Boat::Horizontal(size) => {
                if size + pos.1 > BSIZE {
                    return Err(Error::OutOfBounds);
                }else if self.boats[size-1] == 0{
                    return Err(Error::BoatCount);
                }
                for i in 0..size{
                    if self.data[pos.0][pos.1+i] == 1 {
                        return Err(Error::Overlap);
                    }
                    self.data[pos.0][pos.1+i] = 1;
                }
            }
        }
        self.boats[boat.get_size()-1] -= 1;
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_board() {
        let boats = [4, 3, 2, 1];
        let board = Board::new(&boats);
        assert_eq!(board.boats, boats);
        for row in &board.data {
            for &cell in row {
                assert_eq!(cell, 0);
            }
        }
    }

    #[test]
    fn test_add_boat_success() {
        let boats = [1, 0, 0, 0];
        let mut board = Board::new(&boats);
        let result = board.add_boat(Boat::Horizontal(1), (0, 0));
        assert!(result.is_ok());
        assert_eq!(board.data[0][0], 1);
    }

    #[test]
    fn test_add_boat_out_of_bounds() {
        let boats = [1, 0, 0, 0];
        let mut board = Board::new(&boats);
        let result = board.add_boat(Boat::Horizontal(1), (BSIZE, BSIZE));
        assert!(matches!(result, Err(Error::OutOfBounds)));
    }

    #[test]
    fn test_add_boat_overlap() {
        let boats = [2, 0, 0, 0];
        let mut board = Board::new(&boats);
        board.add_boat(Boat::Horizontal(1), (0, 0)).unwrap();
        let result = board.add_boat(Boat::Horizontal(1), (0, 0));
        assert!(matches!(result, Err(Error::Overlap)));
    }

    #[test]
    fn test_add_boat_touching() {
        let boats = [0, 2, 0, 0];
        let mut board = Board::new(&boats);
        board.add_boat(Boat::Horizontal(2), (0, 0)).unwrap();
        let result = board.add_boat(Boat::Horizontal(2), (0, 1));
        assert!(matches!(result, Err(Error::Overlap)));
    }

    #[test]
    fn test_to_string() {
        let boats = [1, 0, 0, 0];
        let mut board = Board::new(&boats);
        board.add_boat(Boat::Horizontal(1), (0, 0)).unwrap();
        let board_str = board.to_string();
        let expected_str = "0 0 0 0\n\
        B - - - - - - - - - - - - - - - - - - - \n\
        - - - - - - - - - - - - - - - - - - - - \n\
        - - - - - - - - - - - - - - - - - - - - \n\
        - - - - - - - - - - - - - - - - - - - - \n\
        - - - - - - - - - - - - - - - - - - - - \n\
        - - - - - - - - - - - - - - - - - - - - \n\
        - - - - - - - - - - - - - - - - - - - - \n\
        - - - - - - - - - - - - - - - - - - - - \n\
        - - - - - - - - - - - - - - - - - - - - \n\
        - - - - - - - - - - - - - - - - - - - - \n\
        - - - - - - - - - - - - - - - - - - - - \n\
        - - - - - - - - - - - - - - - - - - - - \n\
        - - - - - - - - - - - - - - - - - - - - \n\
        - - - - - - - - - - - - - - - - - - - - \n\
        - - - - - - - - - - - - - - - - - - - - \n\
        - - - - - - - - - - - - - - - - - - - - \n\
        - - - - - - - - - - - - - - - - - - - - \n\
        - - - - - - - - - - - - - - - - - - - - \n\
        - - - - - - - - - - - - - - - - - - - - \n\
        - - - - - - - - - - - - - - - - - - - - \n";
        assert_eq!(board_str, expected_str);
    }
}



fn main() {
    println!("Hello, world!");
}


