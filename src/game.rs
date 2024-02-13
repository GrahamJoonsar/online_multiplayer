pub struct GameState {
    pub players: Vec<Player>,
}

impl GameState {
    pub fn new(player_count: usize) -> GameState {
        let mut state: GameState = GameState { players: vec![] };
        for i in 0..player_count {
            state.players.push(Player::new(i));
        }
        return state;
    }

    pub fn to_packet(&self) -> [u8; 1024] {
        let mut buf: [u8; 1024] = [0; 1024];
        buf[0] = self.players.len() as u8;
        
        return buf;
    }
}

// The player is identified with a unique integer ID
// they have cards in their hand, and cards in play on their side
pub struct Player {
    id: usize,
    hand: Vec<Card>,
    side: (u8, u8),
}

impl Player {
    pub fn new(id: usize) -> Player {
        Player {
            id,
            hand: vec![],
            side: (0, 0),
        }
    }
}

// Lists all of the different card types
enum Card {
    Soldier,
    Barrier,
    Bribe,
    Mortar,
}