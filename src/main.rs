#![allow(dead_code)]
#![allow(unused_variables)]

// External Libs
use std::{net::UdpSocket, sync::Arc}; // For networking
use terminal_menu::{button, label, menu, mut_menu, run}; // For user input

// Internal Libs
mod game;
use game::{Card, CardType, GameState, Player};

#[derive(Debug)]
enum UserType {
    Host,
    Client,
}

fn main() {
    let main_menu = menu(vec![
        label("----------------"),
        label("Select User type"),
        label("----------------"),
        button("Host"),
        button("Join"),
    ]);
    run(&main_menu);

    let user_type: UserType = match mut_menu(&main_menu).selected_item_name() {
        "Host" => UserType::Host,
        _ => UserType::Client,
    };

    //let game_state: GameState = GameState::new(2);
    let mut player1: Player = Player::new(0);
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

    player1.add_card(gub);
    player1.add_card(elder);
    player1.select_card();
}
