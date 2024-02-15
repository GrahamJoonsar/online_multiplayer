use crate::game::{Card, CardType};
use std::net::UdpSocket;

pub fn play_card(card: Card) {
    let socket = UdpSocket::bind("0.0.0.0:34254").expect("bind error");
    let mut buf = [0; 13];
    socket
        .send_to(&buf, "50.116.40.226:8080")
        .expect("send error");
    socket.recv_from(&mut buf).expect("recv error");
}

pub fn join() -> usize {
    0
}
