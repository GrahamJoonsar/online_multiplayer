// The player is identified with a unique integer ID
// they have cards in their hand, and cards in play on their side
pub struct Player {
    id: i32,
    hand: Vec<Card>,
    side: Vec<(Card, Card)>,
}

// Lists all of the different card types
enum CardType {
    Soldier,
    Barrier,
    Bribe,
    Mortar,
    Empty,
}

pub struct Card {
    card_type: CardType,
}

impl Card {
    fn play(&self, player: &mut Player) {

    }
}