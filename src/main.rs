#![allow(dead_code)]
#![allow(unused_variables)]

// Internal Libs
mod client;
mod game;
use game::{Player};

fn main() {
    let id = client::join();
    if id == 255 {
        println!("server is full at the moment");
        return;
    }
    let mut player: Player = Player::new(id);
    player.add_card(client::get_card());
    player.add_card(client::get_card());
    player.select_card();
}
