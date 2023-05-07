use std::net::SocketAddr;

#[derive(Debug)]
pub enum GameCommand3T {
    Empty,
    Create(GameCreateInst),
    Connect(GameConnectInst),
    //Disconnect(),
    Move(GameMoveInst),
    //Delete(),
}

#[derive(Debug, Default)]
pub struct GameMoveInst{
    pub board:u64,
    pub player_code:u64,
    pub player_socket:Option<SocketAddr>,
    pub x_cord:usize,
    pub y_cord:usize,
}


#[derive(Debug, Default)]
pub struct GameCreateInst{
    pub board:u64,
    pub player_code:u64,
    pub player_socket:Option<SocketAddr>,
}

#[derive(Debug, Default)]
pub struct GameConnectInst{
    pub board:u64,
    pub player_code:u64,
    pub player_socket:Option<SocketAddr>,
}

const CREATE_COMMAND_BUFFSIZE:usize = 8+8;
const CONNECT_COMMAND_BUFFSIZE:usize = 8+8;
const MOVE_COMMAND_BUFFSIZE:usize = 8+8+1+1;

impl GameCommand3T {
    
    pub fn parse_create_command(&mut self, buff:&[u8], socket:SocketAddr) -> Result<(), &'static str>{
        if buff.len() != CREATE_COMMAND_BUFFSIZE {
            return Err("incorrect buff size");
        }
        let mut holder = GameCreateInst::default();
        holder.board = u64::from_le_bytes(buff[0..8].try_into().expect("right value array"));
        holder.player_code = u64::from_le_bytes(buff[8..16].try_into().expect("right value array"));
        holder.player_socket = Some(socket);
        *self = GameCommand3T::Create(holder);
        Ok(())
    }
    
    pub fn parse_connect_command(&mut self, buff:&[u8], socket:SocketAddr) -> Result<(), &'static str>{
        if buff.len() != CONNECT_COMMAND_BUFFSIZE {
            return Err("incorrect buff size");
        }
        let mut holder = GameConnectInst::default();
        holder.board = u64::from_le_bytes(buff[0..8].try_into().expect("right value array"));
        holder.player_code = u64::from_le_bytes(buff[8..16].try_into().expect("right value array"));
        holder.player_socket = Some(socket);
        *self = GameCommand3T::Connect(holder);
        Ok(())
    }
    
    pub fn parse_move_command(&mut self, buff:&[u8], socket:SocketAddr) -> Result<(), &'static str>{
        if buff.len() < MOVE_COMMAND_BUFFSIZE {
            return Err("Incorrect buffer size");
        }
        let mut holder = GameMoveInst::default();
        holder.board = u64::from_le_bytes(buff[0..8].try_into().expect("right value array"));
        holder.player_code = u64::from_le_bytes(buff[8..16].try_into().expect("right value array"));
        holder.player_socket = Some(socket);
        holder.x_cord = buff[16].into();
        holder.y_cord = buff[17].into();
        
        *self = GameCommand3T::Move(holder);
        Ok(())
    }
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
}

