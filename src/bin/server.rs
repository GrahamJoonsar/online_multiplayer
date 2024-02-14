use std::net::UdpSocket;

pub fn main() {
    let mut socket = UdpSocket::bind("0.0.0.0:8080").expect("failed to bind");
    let mut buf = [0; 4096];
    loop {
        let sock = socket.try_clone().expect("clone failed");
        match socket.recv_from(&mut buf) {
            Ok((amt, src)) => {
                std::thread::spawn(move || {
                    println!("Handling connection from {}", &src);
                    let buf = &mut buf[..amt];
                    buf.reverse();
                    sock.send_to(&buf, &src).expect("error sending");
                });
            }
            Err(err) => {
                eprintln!("Err: {}", err);
            }
        }
    }
}
