#![allow(dead_code)]
#![allow(unused_variables)]

// External Libs
use terminal_menu::{button, label, menu, mut_menu, run}; // For user input

// Internal Libs
mod client;
mod game;
use game::{Card, CardType, GameState, Player};

#[derive(Debug)]
enum UserType {
    Host,
    Client,
}

fn main() {
    let id = client::join();
    //let game_state: GameState = GameState::new(2);
    let mut player: Player = Player::new(id);
    let gub: Card = Card::new(
        String::from("Gub"),
        String::from("Counts as one point"),
        CardType::Gub,
    );
    let elder: Card = Card::new(
        String::from("Esteemed Elder"),
        String::from("1.5 points; Immune to everything except lightning"),
        CardType::Gub,
    );

    player.add_card(gub);
    player.add_card(elder);
    player.select_card();
}
