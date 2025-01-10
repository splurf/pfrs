use std::{
    net::{TcpListener, UdpSocket},
    ops::RangeInclusive,
};

/// Range of dynamic ports.
const DYNAMIC_PORTS: RangeInclusive<u16> = 49152..=65535;

/// Supported protocols.
pub enum Protocol {
    Tcp,
    Udp,
}

impl Protocol {
    /// Method of binding based on the provided protocol.
    pub const fn method<'a>(&self) -> impl Fn(&'a str, u16) -> bool {
        match self {
            Self::Tcp => |addr, port| TcpListener::bind((addr, port)).is_ok(),
            Self::Udp => |addr, port| UdpSocket::bind((addr, port)).is_ok(),
        }
    }
}

/// Find an available dynamic port (49152..=65535).
pub fn find_open_port<S: AsRef<str>>(addr: S, kind: Protocol) -> Option<u16> {
    let addr = addr.as_ref();
    let method = kind.method();

    #[allow(clippy::useless_conversion)]
    DYNAMIC_PORTS.into_iter().find(|&port| method(addr, port))
}
