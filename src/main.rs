use std::collections::HashMap;
use std::net::UdpSocket;

mod connection;
mod tictactoe;
mod test;

use tictactoe::GameCommand3T;
use tictactoe::GameErrors;


use crate::tictactoe::Player;


use crate::tictactoe::GameCreateInst;
use crate::tictactoe::GameConnectInst;
use crate::tictactoe::GameMoveInst;


fn main() {
    let mut games:HashMap<u64,tictactoe::Game> = HashMap::new();
    let socket = UdpSocket::bind("127.0.0.1:50000").expect("socket is being used");
    
    
    loop{
        let command:Option<tictactoe::GameCommand3T> = connection::fill_command(&socket);
        
        match command {
            Some(option) => {
                match option {
                    GameCommand3T::Create(create_cmd) => {
                        match handle_create(&mut games, create_cmd) {
                            Err(()) => {
                                println!("error creating");
                                continue;
                            },
                            Ok(()) => {
                                
                                println!("created succesfully");
                            }
                        }
                    },
                    GameCommand3T::Connect(connect_cmd) => {
                        match handle_connect(&mut games, connect_cmd, &socket) {
                            Err(err_str) => {
                                println!("error connecting :{}", err_str);
                                continue;
                            },
                            Ok(()) => {
                                println!("connected succesfully");
                            }
                        }
                    },
                    GameCommand3T::Move(move_cmd) => {
                        match handle_move(&mut games, move_cmd) {
                            Err(err_str) => {
                                println!("error in move :{}", err_str);
                                continue;
                            },
                            Ok(possible_winner) => {
                                match possible_winner {
                                    None => {
                                        println!("\t\t no winner this move");
                                    }
                                    Some(winner) => {
                                        println!("---------------------------------");
                                        println!("{:?}", winner);
                                        println!("{:?}", winner);
                                        println!("{:?}", winner);
                                        println!("---------------------------------");
                                    }
                                }
                                
                            }
                        }
                    },
                    _ => {
                        
                    }
                }
                
            },
            None => {
                panic!();
            }
        }
    }
}

fn handle_create(games:&mut HashMap<u64,tictactoe::Game>, create_cmd:GameCreateInst) -> Result<(), ()> {
    println!("-----------------------------------------");
    println!("create");
    println!("-----------------------------------------");
    if let Some(_) = games.get(&create_cmd.board){
        return Err(());
    }
    games.insert(create_cmd.board, tictactoe::Game::new(create_cmd.player_code, create_cmd.player_socket.unwrap()));
    
    Ok(())
}

fn handle_connect(games:&mut HashMap<u64,tictactoe::Game>, create_cmd:GameConnectInst, socket:&UdpSocket) -> Result<(), &'static str> {
    println!("-----------------------------------------");
    println!("connect");
    println!("-----------------------------------------");
    match games.get_mut(&create_cmd.board){
        None => {
            return Err("invalid Board Code (board does not exist)");
        },
        Some(game) => {
            game.print();
            match game.connect(create_cmd) {
                Err(error) => {
                    match error {
                        GameErrors::BoardFull => { return Err("board is already full") },
                        GameErrors::PlayerNotOnGame => { return Err("player not on game") },
                        _ => { return Err("programer missed a error code")}
                    }
                },
                Ok(broadcast) => {
                    connection::send_message(socket, broadcast.p0_socket.as_ref().unwrap(), connection::CommandType::Connect, &broadcast.board);
                    println!("{:?}", broadcast);
                }
            }
            game.print();
        }
    }
    Ok(())
}

fn handle_move(games:&mut HashMap<u64,tictactoe::Game>, move_cmd:GameMoveInst) -> Result<Option<Player>, &'static str> {
    println!("-----------------------------------------");
    println!("move");
    println!("-----------------------------------------");
    match games.get_mut(&move_cmd.board){
        None => {
            return Err("invalid Board Code (board does not exist)");
        },
        Some(game) => {
            game.print_board();
            match game.play(move_cmd){
                Ok(potencial_winner_holder) => {
                    match potencial_winner_holder {
                        None => Ok(None),
                        Some(winner) => Ok(Some(winner)),
                    }
                },
                Err(error) => {
                    match error {
                        GameErrors::PlayerNotOnGame => Err("player not on game"),
                        GameErrors::BadIndex => Err("bad index"),
                        _ => Err("programer missed a relevant error type :(")
                    }
                }
            }
        }
    }
}

