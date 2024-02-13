// External Libs
use terminal_menu::{menu, label, button, run, mut_menu}; // For user input

use game::GameState;

// Internal Libs
mod game;

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
        button("Join")
    ]); run(&main_menu);

    
    let user_type: UserType = match mut_menu(&main_menu).selected_item_name() {
        "Host" => UserType::Host,
        _ => UserType::Client,
    };

    let game_state: GameState = GameState::new(2);
}
