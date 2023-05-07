use std::net::SocketAddr;

#[derive(Debug, Default)]
pub struct BroadcastInstructions{
    pub p0_code:u64,
    pub p0_socket:Option<SocketAddr>,
    pub p1_code:u64,
    pub p1_socket:Option<SocketAddr>,
    pub turn: u8,
    pub started: bool,
    pub board: [u8; 9],
    pub winner: u8,
}

pub struct GameResponse3T {}

impl GameResponse3T {
    pub fn build_connect_response(player_code:u64, player_turn:u8, started:u8, board:&[u8]) -> [u8; 64] {
        let mut holder = [0u8; 64];
        holder[0] = u8::from(crate::connection::CommandType::Connect);
        holder[1] = player_turn;
        holder[2] = started;
        holder[3..11].clone_from_slice(&player_code.to_le_bytes());
        holder[11..20].clone_from_slice(board);
        holder
    }
    
    pub fn build_move_response(player_code:u64, player_turn:u8, started:u8, board:&[u8], won:u8) -> [u8; 64] {
        let mut holder = [0u8; 64];
        holder[0] = u8::from(crate::connection::CommandType::Move);
        holder[1] = player_turn;
        holder[2] = started;
        holder[3..11].clone_from_slice(&player_code.to_le_bytes());
        holder[11..20].clone_from_slice(board);
        holder[20] = won;
        holder
    }
}

