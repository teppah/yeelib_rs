use yeelib::YeeClient;
use std::net::TcpStream;
use std::thread::sleep;
use std::time::Duration;
use std::io::{Write, Read};

fn main() -> anyhow::Result<()> {
    let client = YeeClient::new()?;
    client.get_response(Duration::from_millis(6000))?;

    // let bright = "{\"id\":23,\"method\":\"set_bright\",\"params\":[100,\"smooth\",4000]}\r\n";
    // let dark = "{\"id\":23,\"method\":\"set_bright\",\"params\":[1,\"smooth\",4000]}\r\n";
    //
    // let mut ceiling_light = TcpStream::connect("192.168.2.24:55443")?;
    // loop {
    //     let mut buf = [0u8; 128];
    //     ceiling_light.write(dark.as_bytes())?;
    //     ceiling_light.read(&mut buf)?;
    //     println!("{:?}", String::from_utf8(buf.to_vec()).unwrap());
    //     sleep(Duration::from_secs(5));
    //
    //     let mut buf = [0u8; 128];
    //     ceiling_light.write(bright.as_bytes())?;
    //     ceiling_light.read(&mut buf)?;
    //     println!("{:?}", String::from_utf8(buf.to_vec()).unwrap());
    //     sleep(Duration::from_secs(5));
    // }

    Ok(())
}