pub use super::board::*;
use std::io;
use std::net::UdpSocket;



#[derive(Debug, Default)]
pub struct GameMoveInst{
    pub board:u64,
    pub player:GameSquare,
    pub x_cord:usize,
    pub y_cord:usize,
}

impl GameMoveInst {
    
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
        
        self.player = GameSquare::parse_char_sock(buf[1]);
        
        self.x_cord = usize::from(buf[2]-b'1');
        
        self.y_cord = usize::from(buf[3]-b'1');
        
        let x_cord = 0;
        
        println!("{:?}", buf);
    }
    
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
        self.player = GameSquare::parse_char_stdio(user_input.as_bytes()[0usize]);
        
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

