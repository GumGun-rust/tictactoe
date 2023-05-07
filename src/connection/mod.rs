use std::net::UdpSocket;
use std::net::SocketAddr;

use crate::tictactoe;

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum CommandType{
    Create,
    Connect,
    //Disconnect,
    Move,
    //Delete,
}

impl From<CommandType> for u8 {
    fn from(value: CommandType) -> Self {
        use CommandType::*;
        match value {
            Create => 1,
            Connect => 2,
            //Disconnect => 3,
            Move => 4,
            //Delete => 5,
        }
    }
}

impl TryFrom<u8> for CommandType {
    type Error = &'static str;
    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            _x @ 1 => Ok(CommandType::Create),
            _x @ 2 => Ok(CommandType::Connect),
            //_x @ 3 => Ok(CommandType::Disconnect),
            _x @ 4 => Ok(CommandType::Move),
            //_x @ 5 => Ok(CommandType::Delete),
            _ => Err("Not a valid Game Code")
        }
    }
}



pub fn fill_command(socket:&UdpSocket) -> Option<tictactoe::GameCommand3T> {
    let mut buffer = [0; 64];
    let (amt, src) = socket.recv_from(&mut buffer).expect("read should not crash");
    let mut instruction_packet: Option<tictactoe::GameCommand3T> = Some(tictactoe::GameCommand3T::Empty);
    let buffer = &buffer[..amt];
    let command_type = CommandType::try_from(buffer[0]);
    match command_type {
        Err(_) => {
            return None;
        },
        Ok(code) => {
            match code {
                CommandType::Create => {
                    let parse_status = instruction_packet.as_mut().unwrap().parse_create_command(&buffer[1..17], src);
                    match parse_status {
                        Ok(_) => {
                            
                            
                        },
                        Err(_) => {
                            return None;
                        }
                    }
                },
                CommandType::Connect => {
                    let parse_status = instruction_packet.as_mut().unwrap().parse_connect_command(&buffer[1..17], src);
                    match parse_status {
                        Ok(_) => {
                            
                        },
                        Err(_) => {
                            return None;
                        }
                    }
                },
                CommandType::Move => {
                    let parse_status = instruction_packet.as_mut().unwrap().parse_move_command(&buffer[1..19], src);
                    match parse_status {
                        Ok(_) => {
                            
                        },
                        Err(_) => {
                            return None;
                        }
                    }
                },
                //CommandType::Delete => {},
            }
        },
    }
    instruction_packet
    
}

pub fn send_message(local_socket:&UdpSocket, target:&SocketAddr, formated_buffer:&[u8]) -> Result<(), ()> {
    /*
    let mut buffer = [0; 64];
    buffer[0] = u8::from(command);
    buffer[1..raw_buffer.len()+1].clone_from_slice(raw_buffer);
    let send_buffer = &buffer[0..raw_buffer.len()+1];
    let _ = local_socket.send_to(send_buffer, target);
    */
    let _ = local_socket.send_to(formated_buffer, target);
    Ok(())
}

/*
#[cfg(test)]
mod conversion_tests{
    use super::*;
    

    #[derive(Debug, PartialEq, Eq, Clone, Copy)]
    pub enum GameCode{
        Empty = 0,
        TicTacToe = 1
    }

    impl From<GameCode> for u8 {
        fn from(value: GameCode) -> Self {
            use GameCode::*;
            match value {
                Empty => u8::MAX,
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
*/
