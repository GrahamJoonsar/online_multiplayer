#![allow(dead_code)]
#![allow(unused_variables)]

enum Stage {
    MainMenu,
    Lobby,
    InGame,
}

pub struct GameState {
    players: Vec<Player>,
    stage: Stage
}

impl GameState {
    pub fn new(player_count: usize) -> GameState {
        let mut state: GameState = GameState { 
            players: Vec::with_capacity(player_count),
            stage: Stage::MainMenu,
        };
        for i in 0..player_count {
            state.players.push(Player::new(i));
        }
        return state;
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

    pub fn select_card(&self) {
        use terminal_menu::{menu, label, button, run, mut_menu}; // For user input
        let mut card_menu_vec = vec![
            label("---------------------"),
            label("Select a card to play"),
            label("---------------------"),
        ];

        for card in self.hand.iter() {
            card_menu_vec.push(
                button(format!("{}\t{}", card.title, card.description))
            );
        }

        let card_menu = menu(card_menu_vec); run(&card_menu);
        println!("{}", mut_menu(&card_menu).selected_item_name());

    }

    pub fn add_card(&mut self, card: Card) {
        self.hand.push(card);
    }
}

pub enum CardType {
    Gub,
    Barricade,
    Event,
    Hazard,
    Trap,
    Tool,
    Interrupt,
}

// Lists all of the different card types
pub struct Card {
    title: String,
    description: String,
    card_type: CardType,
}

impl Card {
    pub fn new(title: String, description: String, card_type: CardType) -> Card{
        Card {
            title: format!("{: <20}", title), // makes it 20 characters long adding spaces
            description,
            card_type
        }
    }
}