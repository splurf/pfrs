use std::{net::TcpListener, ops::RangeInclusive};

const DYNAMIC_PORTS: RangeInclusive<u16> = 49152..=65535;

/// Find an available dynamic port (49152..=65535).
pub fn find_open_port() -> Option<u16> {
    #[allow(clippy::useless_conversion)]
    DYNAMIC_PORTS
        .into_iter()
        .find(|&port| TcpListener::bind(("localhost", port)).is_ok())
}
