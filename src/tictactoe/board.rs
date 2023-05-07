use super::errors;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum GameSquare{
    #[default]
    Empty,
    Taken(Player)
}


#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Player{
    P0,
    P1,
}

impl GameSquare{
    fn get_value(self) -> i8{
        use GameSquare::*;
        match self {
            Empty => 0,
            Taken(player) => {
                match player {
                    Player::P0 => -1,
                    Player::P1 => 1,
                }
            },
        }
    }
    
}

/*
impl TryFrom<u8> for GameSquare{
    type Error = &'static str;
    
    fn try_from(value: u8) -> Result<Self, Self::Error> {
        use GameSquare::*;
        let holder = match value {
            _x @ 0 => Empty,
            _x @ 1 => P0,
            _x @ 2 => P1,
            _x => return Err("non valid Game Square"),
        };
        Ok(holder)
    }
    
}
*/

#[derive(Default)]
pub struct Board{
    board: [[GameSquare; 3]; 3]
}

impl Board{
    pub fn new() -> Self {
        Self{
            board:[
                [GameSquare::Empty, GameSquare::Empty, GameSquare::Empty],
                [GameSquare::Empty, GameSquare::Empty, GameSquare::Empty],
                [GameSquare::Empty, GameSquare::Empty, GameSquare::Empty],
            ]
        }
    }
    
    #[allow(dead_code)]       //-----------------------------------------unused function
    pub fn index_valid(index:usize) -> bool {
        if index <= 2 {
            true
        } else {
            false
        }
    }
    
    pub fn print(&self) {
        for row in &self.board {
            for element in row {
                let printable = match element {
                    GameSquare::Empty => {
                        "0"
                    },
                    GameSquare::Taken(player) => {
                        match player {
                            Player::P0 => {
                                "@"
                            },
                            Player::P1 => {
                                "#"
                            },
                        }
                    }
                };
                print!("{} ", printable);
            }
            println!("");
        }
        println!();
    }
    
    pub fn play(&mut self, x:usize, y:usize, player:Player) -> Result<Option<Player>, errors::GameErrors> {
        if self.board[y][x] != GameSquare::Empty {
            return Err(errors::GameErrors::SpaceTaken);
        }
        self.board[y][x]=GameSquare::Taken(player);
        Ok(self.check_if_win())
    }
    
    #[allow(dead_code)]       //-----------------------------------------unused function
    pub fn check_if_win(&self) -> Option<Player> {
        let mut win_con:[i8;8] = [0; 8];
        for number in 0..3 {
            win_con[0] += self.board[0][number].get_value();
            win_con[1] += self.board[1][number].get_value();
            win_con[2] += self.board[2][number].get_value();
            win_con[3] += self.board[number][0].get_value();
            win_con[4] += self.board[number][1].get_value();
            win_con[5] += self.board[number][2].get_value();
            win_con[6] += self.board[number][number].get_value();
            win_con[7] += self.board[number][2-number].get_value();
        }
        for number in win_con {
            if number == -3 {
                return Some(Player::P0);
            }
            if number == 3 {
                return Some(Player::P1);
            }
        }
        None
    }
    
    pub fn board_to_simple(&self, board:&mut [u8;9]){
        for y in 0..3 {
            for x in 0..3 {
                board[y*3+x] = match self.board[y][x] {
                    GameSquare::Empty => { 0 },
                    GameSquare::Taken(player) => {
                        match player {
                            Player::P0 => {1},
                            Player::P1 => {2}
                        }
                    }
                }
            }
        }
        //println!("{:?}", board);
    }
}

