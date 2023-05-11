use std::collections::HashMap;
use std::net::UdpSocket;

mod connection;
mod tictactoe;
mod test;

use tictactoe::GameCommand3T;
use tictactoe::GameErrors;

use crate::tictactoe::GameResponse3T;

use crate::tictactoe::Player;


#[tokio::main]
async fn main() {
    let mut games:HashMap<u64,tictactoe::Game> = HashMap::new();
    let socket = UdpSocket::bind("10.0.44.250:50000").expect("socket is being used");
    
    loop{
        let command:Option<tictactoe::GameCommand3T> = connection::fill_command(&socket);
        
        match command {
            Some(option) => {
                match option {
                    GameCommand3T::Create(create_cmd) => {
                        /*
                        println!("{:#?}", create_cmd);
                        */
                        match handle_create(&mut games, create_cmd) {
                            Err(err_str) => {
                                println!("error creating :{}", err_str);
                                continue;
                            },
                            Ok(broadcast_instructions) => {
                                handle_create_response(&socket, broadcast_instructions);
                                println!("created succesfully");
                            }
                        }
                    },
                    GameCommand3T::Connect(connect_cmd) => {
                        match handle_connect(&mut games, connect_cmd) {
                            Err(err_str) => {
                                println!("error connecting :{}", err_str);
                                continue;
                            },
                            Ok(broadcast_instructions) => {
                                handle_connect_response(&socket, broadcast_instructions);
                                println!("connected succesfully");
                            }
                        }
                    },
                    GameCommand3T::Move(move_cmd) => {
                        let board_key = move_cmd.board;
                        match handle_move(&mut games, move_cmd) {
                            Err(err_str) => {
                                println!("error in move :{}", err_str);
                                continue;
                            },
                            Ok((possible_winner, broadcast_instructions)) => {
                                handle_move_response(&socket, broadcast_instructions);
                                match possible_winner {
                                    None => {
                                        println!("\t\t no winner this move");
                                    }
                                    Some(winner) => {
                                        
                                        println!("---------------------------------");
                                        println!("wonnnn!!! {:?}", winner);
                                        println!("---------------------------------");
                                        win_func().await();
                                        games.remove(&board_key);
                                    }
                                }
                            }
                        }
                    },
                    _ => {
                        
                    }
                }
                
            },
            None => {/*panic!();*/}
        }
    }
}

fn handle_create(games:&mut HashMap<u64,tictactoe::Game>, create_cmd:tictactoe::GameCreateInst) -> Result<tictactoe::BroadcastInstructions, &'static str> {
    println!("-----------------------------------------");
    println!("create");
    println!("-----------------------------------------");
    if let Some(_) = games.get(&create_cmd.board){
        return Err("board_already exists");
    }
    games.insert(create_cmd.board, tictactoe::Game::new(create_cmd.player_code, create_cmd.player_socket.unwrap()));
    
    let holder = tictactoe::BroadcastInstructions{
        p0_code: create_cmd.player_code,
        p0_socket: create_cmd.player_socket,
        p1_code:0,
        p1_socket:None,
        board_id: create_cmd.board,
        turn: 4,
        started: false,
        board: [0; 9],
        winner: 0,
    };
    Ok(holder)
}

fn handle_create_response(socket:&UdpSocket, instructions:tictactoe::BroadcastInstructions) {
    if let Some(socket_direction) = instructions.p0_socket.as_ref() {
        let code = instructions.p0_code;
        let started = 0;//u8::from(instructions.started);
        let turn = instructions.turn;
        let board = instructions.board_id;
        let mut buff = [0u8; tictactoe::RESPONSE_SIZE];
        GameResponse3T::build_create_response(&mut buff, code, board, turn, started, &instructions.board);
        let _ = connection::send_message(&socket, socket_direction, &buff);
    }
    
}

fn handle_connect(games:&mut HashMap<u64,tictactoe::Game>, create_cmd:tictactoe::GameConnectInst) -> Result<tictactoe::BroadcastInstructions, &'static str> {
    println!("-----------------------------------------");
    println!("connect");
    println!("-----------------------------------------");
    match games.get_mut(&create_cmd.board){
        None => {
            return Err("invalid Board Code (board does not exist)");
        },
        Some(game) => {
            //game.print();
            match game.connect(create_cmd) {
                Err(error) => {
                    match error {
                        GameErrors::PlayerNotOnGame => Err("player not on game"),
                        _ => Err("programer missed a error code")
                    }
                },
                Ok(broadcast_instructions) => {
                    Ok(broadcast_instructions)
                }
            }
        }
    }
}

fn handle_connect_response(socket:&UdpSocket, instructions:tictactoe::BroadcastInstructions) {
    //println!("{:?}", instructions);
    //let bytes = [0u8; 64];
    let mut buff = [0u8; tictactoe::RESPONSE_SIZE];
    let board = instructions.board_id;
    let started = u8::from(instructions.started);
    if let Some(socket_direction) = instructions.p0_socket.as_ref() {
        let code = instructions.p0_code;
        let turn = match instructions.turn {
            0 | 2 => 1u8,
            _ => 0u8,
        };
        
        GameResponse3T::build_connect_response(&mut buff, code, board, turn, started, &instructions.board);
        let _ = connection::send_message(&socket, socket_direction, &buff);
    }
    if let Some(socket_direction) = instructions.p1_socket.as_ref() {
        let code = instructions.p1_code;
        let turn = match instructions.turn {
            0 | 1 => 1u8,
            _ => 0u8,
        };
        buff.fill(0);
        GameResponse3T::build_connect_response(&mut buff, code, board, turn, started, &instructions.board);
        let _ = connection::send_message(&socket, socket_direction, &buff);
    }
}


fn handle_move(games:&mut HashMap<u64,tictactoe::Game>, move_cmd:tictactoe::GameMoveInst) -> Result<(Option<Player>, tictactoe::BroadcastInstructions), &'static str> {
    println!("-----------------------------------------");
    println!("move");
    println!("-----------------------------------------");
    match games.get_mut(&move_cmd.board){
        None => {
            return Err("invalid Board Code (board does not exist)");
        },
        Some(game) => {
            let holder = match game.play(move_cmd){
                Err(error) => {
                    match error {
                        GameErrors::PlayerNotOnGame => Err("player not on game"),
                        GameErrors::BadIndex => Err("bad index"),
                        _ => Err("programer missed a relevant error type :(")
                    }
                    
                },
                Ok(potencial_winner_holder) => {
                    Ok(potencial_winner_holder)
                },
            };
            game.print_board();
            holder
        }
    }
}

fn handle_move_response(socket:&UdpSocket, instructions:tictactoe::BroadcastInstructions) {
    //println!("{:?}", instructions);
    //let bytes = [0u8; 64];
    let mut buff = [0u8; tictactoe::RESPONSE_SIZE];
    let board_id = instructions.board_id;
    let started = u8::from(instructions.started);
    if let Some(socket_direction) = instructions.p0_socket.as_ref() {
        let code = instructions.p0_code;
        let turn = match instructions.turn {
            0 | 2 => 1u8,
            _ => 0u8,
        };
        let won = match  instructions.winner {
            1 => 1u8,
            _ => 0u8
        };
        GameResponse3T::build_move_response(&mut buff, code, board_id, turn, started, &instructions.board, won);
        let _ = connection::send_message(&socket, socket_direction, &buff);
    }
    if let Some(socket_direction) = instructions.p1_socket.as_ref() {
        let code = instructions.p1_code;
        let turn = match instructions.turn {
            0 | 1 => 1u8,
            _ => 0u8,
        };
        let won = match  instructions.winner {
            2 => 1u8,
            _ => 0u8
        };
        GameResponse3T::build_move_response(&mut buff, code, board_id, turn, started, &instructions.board, won);
        let _ = connection::send_message(&socket, socket_direction, &buff);
    }
}

async fn win_func(){
    //panic!();
}
