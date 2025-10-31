use std::env;
use std::env::VarError;
use std::net::{IpAddr, Ipv4Addr, SocketAddr};
use crate::error::FrontendError;

pub const BACKEND_HOST_KEY: &str = "PIZZERIA_FRONTEND_BACKEND_HOST";
pub const BACKEND_PORT_KEY: &str = "PIZZERIA_FRONTEND_BACKEND_PORT";
const BACKEND_HOST_DEFAULT: Ipv4Addr = Ipv4Addr::new(127, 0, 0, 1);
const BACKEND_PORT_DEFAULT: u16 = 3333;

pub fn backend_socket_addr() -> Result<SocketAddr, FrontendError> {
    let host = match env::var(BACKEND_HOST_KEY) {
        Ok(value) => value.parse::<IpAddr>()
            .map_err(|error| FrontendError::InvalidHost {
                key: BACKEND_HOST_KEY,
                value,
                source: error,
            })?,
        Err(VarError::NotPresent) => IpAddr::V4(BACKEND_HOST_DEFAULT),
        Err(error @ VarError::NotUnicode(_)) => {
            return Err(FrontendError::NotUnicode {
                key: BACKEND_HOST_KEY,
                source: error,
            })
        }
    };

    let port = match env::var(BACKEND_PORT_KEY) {
        Ok(value) => value.parse::<u16>()
            .map_err(|error| FrontendError::InvalidPort {
                key: BACKEND_PORT_KEY,
                value,
                source: error,
            })?,
        Err(VarError::NotPresent) => BACKEND_PORT_DEFAULT,
        Err(error @ VarError::NotUnicode(_)) => {
            return Err(FrontendError::NotUnicode {
                key: BACKEND_PORT_KEY,
                source: error,
            })
        }
    };

    Ok(SocketAddr::new(host, port))
}
