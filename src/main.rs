use std::net::{UdpSocket, Ipv4Addr};

fn main() -> std::io::Result<()> {
    let message = "M-SEARCH * HTTP/1.1\r\n\
    HOST: 239.255.255.250:1982\r\n\
    MAN: \"ssdp:discover\"\r\n\
    ST: wifi_bulb\r\n";

    let local_addr = "0.0.0.0:6969";
    let multicast_addr = "239.255.255.250:1982";

    let mut buf = [0; 512];

    let mut socket = UdpSocket::bind(local_addr)?;

    let sent_size = socket.send_to(message.as_bytes(), multicast_addr)?;
    println!("sent bytes: {}", sent_size);

    while let (amount, received) = socket.recv_from(&mut buf)? {
        println!("Address: {}, size: {}", received, amount);
        println!("--\n{}", String::from_utf8_lossy(&buf));
    }
    Ok(())
}