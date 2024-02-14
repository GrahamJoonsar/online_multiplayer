use std::net::UdpSocket;

fn main() -> std::io::Result<()> {
    {
        let socket = UdpSocket::bind("0.0.0.0:34254").expect("bind error");

        // Receives a single datagram message on the socket. If `buf` is too small to hold
        // the message, it will be cut off.
        let mut buf = [0; 100];
        let mut i: u8 = 0;
        for elem in buf.iter_mut() {
            *elem += i;
            i += 1;
        }
        socket
            .send_to(&buf, "50.116.40.226:8080")
            .expect("send error");
        socket.recv_from(&mut buf)?;
        print!("{:?}", buf);
    } // the socket is closed here
    Ok(())
}
