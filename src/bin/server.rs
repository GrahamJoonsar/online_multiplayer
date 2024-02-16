use alkahest::{alkahest, deserialize, serialize_to_vec};
use rand::Rng;
use std::net::{SocketAddr, UdpSocket};
use std::sync::{Arc, Mutex, OnceLock};

#[path = "../game.rs"]
mod game;
use game::{CardType, GameState};

//static state: GameState;
pub const MAX_PLAYERS: usize = 200;

#[alkahest(Formula, SerializeRef, Deserialize)]
struct SerializedCard {
    title: [u8; 20],
    description: [u8; 128],
    card_type: CardType,
}

pub fn main() {
    let state = Arc::new(Mutex::<GameState>::new(GameState::new(MAX_PLAYERS)));
    //state = GameState::new(2);
    let socket = UdpSocket::bind("0.0.0.0:8080").expect("failed to bind");
    let mut buf = [0; 4096];
    loop {
        let sock = socket.try_clone().expect("clone failed");
        match socket.recv_from(&mut buf) {
            Ok((amt, src)) => {
                let state = Arc::clone(&state);
                std::thread::spawn(move || {
                    println!("Handling connection from {}", &src);
                    let buf = &mut buf[..amt];
                    if (buf[0] == 1) {
                        println!("join request from {}", &src);
                        join(&sock, &src, &state);
                    }
                    if (buf[0] == 2) {
                        get_card(&sock, &src, &state);
                    }
                    //buf.reverse();
                    //sock.send_to(&buf, &src).expect("error sending");
                });
            }
            Err(err) => {
                eprintln!("Err: {}", err);
            }
        }
    }
}

fn join(socket: &UdpSocket, addr: &SocketAddr, state: &Arc<Mutex<GameState>>) {
    let mut data = state.lock().unwrap();
    for index in 0..MAX_PLAYERS {
        if !data.is_active(index) {
            data.add_player(index, socket);
            let mut buf: [u8; 1] = [index.try_into().unwrap(); 1];
            socket.send_to(&buf, addr).expect("error sending");
            break;
        }
    }
    println!("lobby full");
    let mut buf: [u8; 1] = [255; 1];
    socket.send_to(&buf, addr).expect("error sending");
}

fn get_card(socket: &UdpSocket, addr: &SocketAddr, state: &Arc<Mutex<GameState>>) {
    let num = rand::thread_rng().gen_range(0..2);
    if (num == 0) {
        //let mut data = state.lock().unwrap();
        let value = new_card("Gub", "Counts as one point", CardType::Gub);
        let mut data = Vec::new();
        let (size, _) = serialize_to_vec::<SerializedCard, _>(&value, &mut data);
        socket.send_to(&data, addr).expect("error sending");
    } else {
        let value = new_card(
            "Esteemed Elder",
            "1.5 points; Immune to everything except lightning",
            CardType::Gub,
        );
        let mut data = Vec::new();
        let (size, _) = serialize_to_vec::<SerializedCard, _>(&value, &mut data);
        socket.send_to(&data, addr).expect("error sending");
    }
}

fn new_card(name: &str, description: &str, card_type: CardType) -> SerializedCard {
    let mut card = SerializedCard {
        title: [' ' as u8; 20],
        description: [0; 128],
        card_type,
    };
    let mut i: usize = 0;
    for e in name.bytes() {
        card.title[i] = e;
        i += 1;
    }
    let s2 = "Counts as one point";
    i = 0;
    for e in description.bytes() {
        card.description[i] = e;
        i += 1;
    }
    card
}
