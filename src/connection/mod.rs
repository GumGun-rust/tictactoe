use std::net::UdpSocket;
use std::io;

use super::tictactoe;

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
enum GameCode{
    TicTacToe = 1
}

impl From<GameCode> for u8 {
    fn from(value: GameCode) -> Self {
        use GameCode::*;
        match value {
            TicTacToe => 1
        }
    }
}

impl TryFrom<u8> for GameCode {
    type Error = &'static str;
    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            _x @ 1 => Ok(GameCode::TicTacToe),
            _ => Err("Not a valid Game Code")
        }
    }
}

enum TicTacToeOption {
    Create(),
    Delete(Option<i32>),
    Command(Option<tictactoe::GameMoveInst>),
}

enum GameDeletion {
    TicTacToe(i32),
}

enum GameCommand {
    TicTacToe(tictactoe::GameMoveInst),
}

pub enum CommandList{
    Empty,
    Create(),
    Delete(Option<i32>),
    Command(Option<tictactoe::GameMoveInst>),
}


/*
impl CommandList {
    pub fn to_u8(self) -> u8 {
        use CommandList::*;
        match self {
            Empty => 0u8,
            Create(_) => 1u8,
            Delete(_) => 2u8,
            Command(_) => 3u8,
        }
    }
    
    pub fn from_u8(value: u8) -> Self {
        use CommandList::*;
        match value {
            test @ 0 => Empty,
            test @ 1 => Create(None),
            test @ 2 => Delete(None),
            test => Command(None),
        }
    }
}
*/

#[derive(Debug)]
pub enum Command1 {
    Empty1,
    TicTacToe(GameCommand3T),
    Newgame(GameCommand3T)
}

#[derive(Debug)]
enum GameCommand3T {
    Empty,
    Create(),
    Delete(),
    Move(),
}

impl TryFrom<u8> for Command1 {
    type Error = &'static str;
    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            _x @ 1 => Ok(Command1::TicTacToe(GameCommand3T::Empty)),
            _x @ 2 => Ok(Command1::Newgame(GameCommand3T::Empty)),
            _ => Err("Not a valid Game Code")
        }
    }
}



impl Command1 {
    fn get_value(self) -> i8 {
        panic!();
    }
}

pub fn fill_command(socket:&UdpSocket) -> Command1 {
    
    let mut raw_buffer = [0; 64];
    let (amt, _src) = socket.recv_from(&mut raw_buffer).expect("hola");
    let buffer = &mut raw_buffer[..amt-1]; //removes the enter
    println!("{:?}", &buffer);
    println!("{}", b'1');
    println!("{:?}", buffer);
    let buffer:Vec<u8> = buffer.iter().map(| x | x - b'1' ).collect();
    
    let command = Command1::try_from(buffer[0]);
    println!("{:?}", &command);
    println!("{:?}", &buffer);
    
    
    panic!();
}


impl CommandList {
    
    pub fn fill_from_web_test(&mut self, socket:&UdpSocket) {
        let mut raw_buffer = [0; 64];
        let (amt, _src) = socket.recv_from(&mut raw_buffer).expect("hola");
        let buffer = &mut raw_buffer[..amt-1]; //removes the enter
        println!("{:?}", &buffer);
        println!("{}", b'1');
        println!("{:?}", buffer);
        let buffer:Vec<u8> = buffer.iter().map(| x | x - b'1' ).collect();
        
        println!("{:?}", &buffer);
        panic!();
    }
}

#[cfg(test)]
mod conversion_tests{
    use super::*;
    
    #[test]
    #[allow(non_snake_case)]
    fn u8_conv_TicTacToe() {
        let game_code = GameCode::TicTacToe;
        let numeric_code = u8::from(game_code);
        println!("{:?} = 1", game_code);
        assert_eq!(numeric_code, 1);
        let conv_game_code = GameCode::try_from(numeric_code).expect("Should be a valid convertion");
        assert_eq!(game_code, conv_game_code);
    }
    
    #[test]
    #[allow(non_snake_case)]
    fn invalid_conv() {
        let game_code = GameCode::try_from(u8::MAX);
        if let Ok(_) = game_code {
            panic!();
        }
    }
    
}

