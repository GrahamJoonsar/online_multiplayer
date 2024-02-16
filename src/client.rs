use crate::game::{Card, CardType};
use alkahest::{alkahest, deserialize};
use std::net::UdpSocket;

pub fn get_card() -> Card {
    #[alkahest(Formula, SerializeRef, Deserialize)]
    struct SerializedCard {
        title: [u8; 20],
        description: [u8; 128],
        card_type: CardType,
    }
    const SERIALIZED_LEN: usize = 164;

    let mut data = Vec::<u8>::with_capacity(SERIALIZED_LEN);
    for _ in 0..SERIALIZED_LEN {
        data.push(0);
    }

    let socket = UdpSocket::bind("0.0.0.0:34254").expect("bind error");
    let buf = [2; 1];
    socket
        .send_to(&buf, "50.116.40.226:8080")
        .expect("send error");
    socket.recv_from(&mut data).expect("recv error");

    let de = deserialize::<SerializedCard, SerializedCard>(&data[..SERIALIZED_LEN]).unwrap();

    Card::new(
        std::str::from_utf8(&de.title)
            .expect("bad string")
            .to_string(),
        std::str::from_utf8(&de.description)
            .expect("bad string")
            .to_string(),
        de.card_type,
    )
}

pub fn play_card(card: Card) {
    let socket = UdpSocket::bind("0.0.0.0:34254").expect("bind error");
    let mut buf = [3; 1];
    socket
        .send_to(&buf, "50.116.40.226:8080")
        .expect("send error");
    socket.recv_from(&mut buf).expect("recv error");
}

pub fn join() -> usize {
    let socket = UdpSocket::bind("0.0.0.0:34254").expect("bind error");
    let mut buf = [1; 1];
    socket
        .send_to(&buf, "50.116.40.226:8080")
        .expect("send error");
    socket.recv_from(&mut buf).expect("recv error");
    buf[0] as usize
}
