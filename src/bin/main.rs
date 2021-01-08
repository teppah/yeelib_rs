use std::time::Duration;

use yeelib::YeeClient;
use std::net::TcpStream;
use std::thread::sleep;
use std::io::{Write, Read};
use yeelib::err::YeeError;
use yeelib::light::Light;

fn main() -> anyhow::Result<()> {
    let client = YeeClient::new()?;
    let mut res: Vec<Light> = loop {
        let lights = client.get_response(Duration::from_secs(1));
        if lights.len() == 0 {
            println!("zero");
        } else {
            break lights;
        }
    };
    let light = res.get_mut(0).unwrap();
    println!("{:?}", light);

    loop {
        light.set_bright(1);
        sleep(Duration::from_secs(4));
        light.set_bright(100);
        sleep(Duration::from_secs(4));
    }

    // let bright = "{\"id\":23,\"method\":\"set_bright\",\"params\":[100,\"smooth\",400]}\r\n";
    // let dark = "{\"id\":23,\"method\":\"set_bright\",\"params\":[1,\"smooth\",400]}\r\n";
    // let mut ceiling_light = TcpStream::connect("192.168.2.24:55443")?;
    // loop {
    //     let mut buf = [0u8; 128];
    //     ceiling_light.write(dark.as_bytes())?;
    //     ceiling_light.read(&mut buf)?;
    //     println!("{}", String::from_utf8(buf.to_vec()).unwrap());
    //     sleep(Duration::from_secs(5));
    //
    //     let mut buf = [0u8; 128];
    //     ceiling_light.write(bright.as_bytes())?;
    //     ceiling_light.read(&mut buf)?;
    //     println!("{}", String::from_utf8(buf.to_vec()).unwrap());
    //     sleep(Duration::from_secs(5));
    // }

    Ok(())
}