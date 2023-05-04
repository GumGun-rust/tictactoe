use std::net::UdpSocket;
use std::io;

use super::tictactoe;

pub enum CommandList{
    Empty,
    Create(Option<i32>),
    Delete(Option<i32>),
    Move(Option<GameMoveInst>),
}

#[derive(Debug, Default)]
pub struct GameMoveInst{
    pub board:u64,
    pub player:tictactoe::GameSquare,
    pub x_cord:usize,
    pub y_cord:usize,
}

impl CommandList{
    pub fn to_u8(self) -> u8 {
        use CommandList::*;
        match self {
            Empty => 0u8,
            Create(_) => 1u8,
            Delete(_) => 2u8,
            Move(_) => 3u8,
        }
    }
    
    pub fn from_u8(value: u8) -> Self {
        use CommandList::*;
        match value {
            test @ 0 => Empty,
            test @ 1 => Create(None),
            test @ 2 => Delete(None),
            test => Move(None),
        }
    }
    
}

pub fn test(){
    println!("test");
}


impl GameMoveInst{
    pub fn new() -> Self {
        Self::default()
    }
    
    #[allow(dead_code)]
    pub fn fill_from_web(&mut self, socket:&UdpSocket) {
        let mut buf = [0; 64];
        let (amt, src) = socket.recv_from(&mut buf).expect("hola");
        
        let buffer = &mut buf[..amt];
        self.board = u64::from(buf[0]-b'1');
        //println!("{:?}", board/*-0x3131313131313131usize*/);
        
        self.player = tictactoe::GameSquare::parse_char_sock(buf[1]);
        
        self.x_cord = usize::from(buf[2]-b'1');
        
        self.y_cord = usize::from(buf[3]-b'1');
        
        let x_cord = 0;
        
        println!("{:?}", buf);
    }
        /*
        let board = usize::from_ne_bytes(buf[0..8].try_into().unwrap());
        */
    
    #[allow(dead_code)]
    fn fill_from_stdio(&mut self) {
        let mut user_input = String::new();
        let stdin = io::stdin();
        
        println!("Board: ");
        stdin.read_line(&mut user_input).expect("text");
        user_input.pop();
        self.board = user_input.parse::<u64>().expect("boardName");
        
        user_input.clear();
        println!("player: ");
        stdin.read_line(&mut user_input).expect("text");
        self.player = tictactoe::GameSquare::parse_char_stdio(user_input.as_bytes()[0usize]);
        
        user_input.clear();
        println!("X: ");
        stdin.read_line(&mut user_input).unwrap();
        user_input.pop();
        self.x_cord = user_input.parse::<usize>().expect("positive Number");
        
        user_input.clear();
        println!("Y: ");
        stdin.read_line(&mut user_input).unwrap();
        user_input.pop();
        self.y_cord = user_input.parse::<usize>().expect("positive Number");
    }
    
}
