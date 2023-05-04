use std::collections::HashMap;
use std::net::UdpSocket;
use std::io;

mod commands;
mod tictactoe;


fn main() {
    panic!();
    let mut games:HashMap<u64,tictactoe::Game> = HashMap::new();
    games.insert(00,tictactoe::Game::new(0,1));
    games.insert(01,tictactoe::Game::new(0,1));
    games.insert(02,tictactoe::Game::new(0,1));
    games.insert(03,tictactoe::Game::new(0,1));
    let socket = UdpSocket::bind("127.0.0.1:50000").unwrap();
    /*
    let mut buf = [0; 64];
    let (amt, src) = socket.recv_from(&mut buf).unwrap();
    let hola = buf as u64;
    */
    
    
    //let mut game = tictactoe::Game::new(0,1);
    //game.print();
    let mut game_move = commands::GameMoveInst::new();
    let mut game_command = commands::CommandList::Empty;
    
    loop{
        game_move.fill_from_web(&socket);
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
