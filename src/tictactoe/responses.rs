use std::net::SocketAddr;

#[derive(Debug, Default)]
pub struct BroadcastInstructions{
    pub p0_code:u64,
    pub p0_socket:Option<SocketAddr>,
    pub p1_code:u64,
    pub p1_socket:Option<SocketAddr>,
    pub board_id: u64,
    pub turn: u8,
    pub started: bool,
    pub board: [u8; 9],
    pub winner: u8,
}

pub struct GameResponse3T {}

pub const RESPONSE_SIZE:usize = 29usize;
impl GameResponse3T {
    pub fn build_create_response(buffer:&mut[u8], player_code:u64, board_id:u64, player_turn:u8, started:u8, board:&[u8]) {
        buffer[0] = u8::from(crate::connection::CommandType::Create);
        buffer[1] = player_turn;
        buffer[2] = started;
        buffer[3..11].clone_from_slice(&player_code.to_le_bytes());
        buffer[11..19].clone_from_slice(&board_id.to_le_bytes());
        buffer[19..28].clone_from_slice(board);
        println!("creation response \n\t{:?}", buffer);
    }
    
    pub fn build_connect_response(buffer:&mut[u8], player_code:u64, board_id:u64, player_turn:u8, started:u8, board:&[u8]) {
        buffer[0] = u8::from(crate::connection::CommandType::Connect);
        buffer[1] = player_turn;
        buffer[2] = started;
        buffer[3..11].clone_from_slice(&player_code.to_le_bytes());
        buffer[11..19].clone_from_slice(&board_id.to_le_bytes());
        buffer[19..28].clone_from_slice(board);
        println!("connect response \n\t{:?}", buffer);
        //holder[11..20].clone_from_slice(board);
    }
    
    pub fn build_move_response(buffer:&mut[u8], player_code:u64, board_id:u64, player_turn:u8, started:u8, board:&[u8], won:u8) {
        buffer[0] = u8::from(crate::connection::CommandType::Move);
        buffer[1] = player_turn;
        buffer[2] = started;
        buffer[3..11].clone_from_slice(&player_code.to_le_bytes());
        buffer[11..19].clone_from_slice(&board_id.to_le_bytes());
        buffer[19..28].clone_from_slice(board);
        buffer[28] = won;
    }
}

