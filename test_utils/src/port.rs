use std::net::{Ipv4Addr, SocketAddrV4, TcpListener};

pub type Port = u16;

pub fn get_free_port() -> Port {
    loop {
        let addr = SocketAddrV4::new(Ipv4Addr::LOCALHOST, 0);
        if let Ok(listener) = TcpListener::bind(addr) {
            if let Ok(bind) = listener.local_addr() {
                return bind.port();
            }
        }
    }
}
