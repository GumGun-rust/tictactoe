mod board;
mod commands;
mod errors;
mod responses;
//local modules
use std::net::SocketAddr;

pub use crate::tictactoe::responses::GameResponse3T;

pub use crate::tictactoe::responses::BroadcastInstructions;

pub use commands::*;
pub use board::*;
pub use errors::*;

pub struct Game{
    pub started: bool,
    pub last: Option<Player>,
    pub p0_connected:bool,
    pub p0_code:u64,
    pub p0_socket:Option<SocketAddr>,
    pub p1_connected:bool,
    pub p1_code:u64,
    pub p1_socket:Option<SocketAddr>,
    pub board: Board,
    pub winner: Option<Player>,
}

impl Game{
    pub fn new(p0_code:u64, p0_socket:SocketAddr) -> Self {
        Self{
            board:Board::new(),
            started: false,
            p0_connected:true,
            p0_code,
            p0_socket:Some(p0_socket),
            p1_connected:false,
            p1_code:0,
            p1_socket:None,
            last:None,
            winner:None,
        }
    }
    
    pub fn connect(&mut self, args:GameConnectInst) -> Result<BroadcastInstructions, GameErrors> {
        /*
        pub board:u64,
        pub player_code:u64,
        pub player_socket:Option<SocketAddr>,
        */
        if !self.started {
            if self.p0_connected {
                if self.p1_connected {
                    return Err(GameErrors::BoardFull);
                } else {
                    self.p1_code = args.player_code;
                    self.p1_socket = args.player_socket;
                    self.p1_connected = true;
                }
            }else {
                self.p0_code = args.player_code;
                self.p0_socket = args.player_socket;
                self.p0_connected = true;
            }
            
            let mut holder = responses::BroadcastInstructions{
                p0_code:self.p0_code,
                p0_socket:self.p0_socket.clone(),
                p1_code:self.p1_code,
                p1_socket:self.p1_socket.clone(),
                turn: self.check_turn(),
                started: self.started,
                board: [0; 9],
                winner: 0,
            };
            self.board.board_to_simple(&mut holder.board);
            Ok(holder)
        } else {
            
            let player = match self.player_from_id(args.player_code) {
                Some(player) => player,
                None => { return Err(GameErrors::PlayerNotOnGame) }
            };
            
            match player {
                Player::P0 => {
                    self.p0_socket = args.player_socket;
                }
                Player::P1 => {
                    self.p1_socket = args.player_socket;
                }
            }
            
            let mut holder = responses::BroadcastInstructions{
                p0_code:self.p0_code,
                p0_socket:self.p0_socket.clone(),
                p1_code:self.p1_code,
                p1_socket:self.p1_socket.clone(),
                turn: self.check_turn(),
                started: self.started,
                board: [0; 9],
                winner: 0,
            };
            self.board.board_to_simple(&mut holder.board);
            Ok(holder)
        }
        
    }
    
    fn check_turn(&self) -> u8{
        match self.last {
            None => 0,
            Some(player) => {
                match player {
                    Player::P0 => 1,
                    Player::P1 => 2,
                }
            }
        }
    }
    
    fn check_winner(&self) -> u8 {
        let remove_me = match self.winner {
            None => 0,
            Some(player) => {
                match player {
                    Player::P0 => 1,
                    Player::P1 => 2,
                }
            }
        };
        println!("this is a debug line {}", remove_me);
        remove_me
    }
    
    pub fn play(&mut self, args:GameMoveInst) -> Result<(Option<Player>, BroadcastInstructions), GameErrors> {
        /*
        pub board:u64,
        pub player_code:u64,
        pub player_socket:Option<SocketAddr>,
        pub x_cord:usize,
        pub y_cord:usize,
        */
        let player = match self.player_from_id(args.player_code) {
            Some(player) => player,
            None => { return Err(GameErrors::PlayerNotOnGame) }
        };
        if !self.started {
            if !self.p0_connected || !self.p1_connected {
                return Err(GameErrors::LobbyNotFull);
            }
            
        } else {
            if self.last == Some(player) {
                return Err(GameErrors::NotTurn);
            }
        }
        if !Board::index_valid(args.x_cord) || !Board::index_valid(args.y_cord) {
            return Err(GameErrors::BadIndex);
        }
        
        let response_holder = self.board.play(args.x_cord, args.y_cord, player)?;
        self.last = Some(player);
        self.started = true;
        self.winner = response_holder;
        
        let mut holder = responses::BroadcastInstructions{
            p0_code:self.p0_code,
            p0_socket:self.p0_socket.clone(),
            p1_code:self.p1_code,
            p1_socket:self.p1_socket.clone(),
            turn: self.check_turn(),
            started: self.started,
            board: [0; 9],
            winner: self.check_winner(),
        };
        self.board.board_to_simple(&mut holder.board);
        Ok((response_holder, holder))
    }
    
    fn player_from_id(&self, player_id:u64) -> Option<Player> {
        if player_id == self.p0_code {
            return Some(Player::P0);
        }
        if player_id == self.p1_code {
            return Some(Player::P1);
        }
        None
    }
    
    #[allow(dead_code)]
    pub fn print(&self) {
        dbg!(self.started);
        dbg!(self.p0_connected);
        dbg!(self.p0_code);
        dbg!(self.p0_socket);
        dbg!(self.p1_connected);
        dbg!(self.p1_code);
        dbg!(self.p1_socket);
    }
    
    pub fn print_board(&self) {
        self.board.print();
    }
    
}

