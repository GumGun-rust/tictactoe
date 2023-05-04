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
    board: [[GameSquare; 3]; 3]
}

pub struct Game{
    board: Board,
    p0:u64,
    p1:u64,
    last: GameSquare,
}

pub enum GameErrors{
    SpaceTaken,
    NotTurn,
    BadIndex,
}


impl Game{
    pub fn new(p0:u64, p1:u64) -> Self {
        Self{
            board:Board::new(),
            p0,
            p1,
            last:GameSquare::Empty
        }
    }
    
    pub fn play(&mut self, x:usize, y:usize, player:GameSquare) -> Result<Option<GameSquare>, Option<GameErrors>> {
        if self.last == player {
            return Err(Some(GameErrors::NotTurn));
        }
        if !Board::index_valid(x) || !Board::index_valid(y) {
            return Err(Some(GameErrors::BadIndex));
        }
        self.last = player;
        self.board.play(x, y, player)
    }
    
    pub fn print(&self) {
        self.board.print();
    }
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
    
    pub fn index_valid(index:usize) -> bool {
        if index < 2 {
            true
        } else {
            false
        }
    }
    
    pub fn print(&self) {
        use GameSquare::*;
        for row in &self.board {
            for element in row {
                let printable = match element {
                    P0 => {
                        "@"
                    },
                    P1 => {
                        "#"
                    },
                    Empty => {
                        "0"
                    },
                };
                print!("{} ", printable);
            }
            println!("");
        }
        println!();
    }
    
    
    pub fn play(&mut self, x:usize, y:usize, player:GameSquare) -> Result<Option<GameSquare>, Option<GameErrors>> {
        if player == GameSquare::Empty {
            return Err(None);
        }
        if self.board[y][x] != GameSquare::Empty {
            return Err(Some(GameErrors::SpaceTaken));
        }
        self.board[y][x]=player;
        Ok(self.check_if_win())
    }
    
    pub fn check_if_win(&self) -> Option<GameSquare> {
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
                return Some(GameSquare::P0);
            }
            if number == 3 {
                return Some(GameSquare::P1);
            }
        }
        None
    }
}

