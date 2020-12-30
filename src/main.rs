use yeelib::YeeClient;
use std::time::Duration;

fn main() -> std::io::Result<()> {
    let client = YeeClient::new()?;
    client.get_response(Duration::from_millis(6000))?;
    Ok(())
}