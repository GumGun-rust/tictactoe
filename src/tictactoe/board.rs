

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum GameSquare{
    #[default]
    Empty,
    P0,
    P1,
}

impl GameSquare{
    pub fn get_value(self) -> i8{
        use GameSquare::*;
        match self {
            P0 => -1,
            P1 => 1,
            Empty => 0
        }
    }
    pub fn parse_char_stdio(number: u8) -> GameSquare {
        if number == b'X' {
            return Self::P1;
        }
        if number == b'O' {
            return Self::P0;
        }
        panic!();
    }
    
    pub fn parse_char_sock(number: u8) -> GameSquare {
        if number == b'1' {
            return GameSquare::P1;
        }
        if number == b'0' {
            return GameSquare::P0;
        }
        panic!();
    }
}

pub struct Board{
    pub board: [[GameSquare; 3]; 3]
}

pub struct Game{
    pub board: Board,
    pub p0:u64,
    pub p1:u64,
    pub last: GameSquare,
}

