use std::net::UdpSocket;

fn main() {
    let socket = UdpSocket::bind("127.0.0.1:0").expect("Could not bind client socket");
    socket.connect("127.0.0.1:2237").expect("Could not connect to server");

    socket.send(b"Hello, server!").expect("Failed to write to server");
}