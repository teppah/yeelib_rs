use std::net::{Ipv4Addr, SocketAddrV4};
use std::io;
use socket2::{Socket, Domain, Type, Protocol, SockAddr};

pub const MULTICAST_ADDR: Ipv4Addr = Ipv4Addr::new(239, 255, 255, 250);
pub const MULTICAST_PORT: u16 = 1982;
pub const ALL_LOCAL: Ipv4Addr = Ipv4Addr::new(0, 0, 0, 0);
pub const DEFAULT_LOCAL_PORT: u16 = 7821;

pub const SEARCH_MSG: &[u8] = b"\
    M-SEARCH * HTTP/1.1\r\n\
    HOST: 239.255.255.250:1982\r\n\
    MAN: \"ssdp:discover\"\r\n\
    ST: wifi_bulb";

#[derive(Debug)]
pub struct YeeClient {
    seeker: Socket,
    multicast_addr: SocketAddrV4,
}

impl YeeClient {
    pub fn new() -> io::Result<YeeClient> {
        let addr = SocketAddrV4::new(MULTICAST_ADDR, MULTICAST_PORT);
        Self::with_addr(addr, DEFAULT_LOCAL_PORT)
    }
    
    pub fn with_addr(multicast_addr: SocketAddrV4, local_port: u16) -> io::Result<YeeClient> {
        let socket = Socket::new(Domain::ipv4(),
                                 Type::dgram(),
                                 Some(Protocol::udp()))?;
        socket.join_multicast_v4(multicast_addr.ip(), &Ipv4Addr::UNSPECIFIED)?;
        // we don't know the IPs of the lights, so listen to all traffic
        socket.bind(&SockAddr::from(SocketAddrV4::new(ALL_LOCAL, local_port)))?;


        Ok(YeeClient { seeker: socket, multicast_addr })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn is_multicast() {
        assert!(MULTICAST_ADDR.is_multicast());
    }

    #[test]
    fn create_valid_client() {
        // given
        let other_multicast_addr = Ipv4Addr::new(237, 220, 1, 32);
        let other_multicast_port = 1235;
        let sock_addr = SocketAddrV4::new(other_multicast_addr, other_multicast_port);
        let local_port = 5435;

        // when
        let client = YeeClient::with_addr(sock_addr, local_port);

        // then
        assert!(client.is_ok());
        let client = client.unwrap();
        assert_eq!(client.multicast_addr, sock_addr);

        let local_addr = client.seeker.local_addr();
        assert!(local_addr.is_ok());
        let local_addr = local_addr.unwrap().as_inet().unwrap();
        assert_eq!(local_addr.port(), local_port);
    }

    #[test]
    fn create_default_client() {
        // when
        let client = YeeClient::new();

        // then
        assert!(client.is_ok());
        let client = client.unwrap();

        assert_eq!(client.multicast_addr.ip(), &MULTICAST_ADDR);
        assert_eq!(client.multicast_addr.port(), MULTICAST_PORT);

        let local_addr = client.seeker.local_addr();
        assert!(local_addr.is_ok());
        let local_addr = local_addr.unwrap().as_inet().unwrap();
        assert_eq!(local_addr.port(), DEFAULT_LOCAL_PORT);
    }

    #[test]
    fn create_with_invalid_multicast() {
        // given
        let invalid_multicast = SocketAddrV4::new(
            Ipv4Addr::new(223, 0, 0, 255), 80);

        // when
        let client = YeeClient::with_addr(invalid_multicast, 1234);

        // then
        assert!(client.is_err());
    }
}


