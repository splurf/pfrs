use std::{
    net::{IpAddr, TcpListener, UdpSocket},
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
    pub const fn method<A: Into<IpAddr>>(&self) -> impl Fn(A, u16) -> bool {
        match self {
            Self::Tcp => |addr: A, port| TcpListener::bind((addr.into(), port)).is_ok(),
            Self::Udp => |addr: A, port| UdpSocket::bind((addr.into(), port)).is_ok(),
        }
    }
}

/// Find an available dynamic port (49152..=65535).
pub fn find_open_port<A: Into<IpAddr>>(addr: A, kind: Protocol) -> Option<u16> {
    let addr = addr.into();
    let method = kind.method();

    #[allow(clippy::useless_conversion)]
    DYNAMIC_PORTS.into_iter().find(|&port| method(addr, port))
}
