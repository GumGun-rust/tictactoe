use std::collections::HashMap;
use std::net::UdpSocket;

use crate::tictactoe::GameMoveInst;

mod connection;
mod tictactoe;


fn main() {
    let mut games:HashMap<u64,tictactoe::Game> = HashMap::new();
    games.insert(00,tictactoe::Game::new(0,1));
    games.insert(01,tictactoe::Game::new(0,1));
    games.insert(02,tictactoe::Game::new(0,1));
    games.insert(03,tictactoe::Game::new(0,1));
    let socket = UdpSocket::bind("127.0.0.1:50000").unwrap();
    
    connection::fill_command(&socket);
    
    //let mut game = tictactoe::Game::new(0,1);
    //game.print();
    let mut game_move = tictactoe::GameMoveInst::new();
    let mut game_command = connection::CommandList::Empty;
    
    loop{
        game_command.fill_from_web_test(&socket);
        //game_move.fill_from_web(&socket);
        //game_move.fill_from_stdio();
        println!("{:?}", game_move);
        
        /*
        match game_command {
            CommandList::Empty => {},
            CommandList::Create => {},
            CommandList::Delete => {},
            CommandList::Move(_) => {},
        }
        */
        
        if let Some(game) = games.get_mut(&game_move.board) {
            
            match game.play(game_move.x_cord, game_move.y_cord, game_move.player) {
                Ok(posible_winner) => {
                    match posible_winner {
                        Some(winner) => { 
                            println!("gano {:?}", winner);
                            
                        },
                        None => {}
                    }
                },
                Err(error) => {
                    match error {
                        Some(handleable_error) => {
                            match handleable_error {
                                tictactoe::GameErrors::SpaceTaken => {
                                    None::<u8>.expect("spaceTaken");
                                },
                                tictactoe::GameErrors::NotTurn => {
                                    None::<u8>.expect("NotTurn");
                                },
                                tictactoe::GameErrors::BadIndex => {
                                    None::<u8>.expect("BadIndex");
                                },
                                
                            }
                        },
                        None => {
                            panic!();
                        }
                    }
                }
            }
            game.print();
        }
    }
}
