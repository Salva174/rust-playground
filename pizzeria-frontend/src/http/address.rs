use std::env;
use std::net::{IpAddr, Ipv4Addr, SocketAddr};
use crate::error::FrontendError;

pub const BACKEND_HOST_KEY: &str = "PIZZERIA_FRONTEND_BACKEND_HOST";
pub const BACKEND_PORT_KEY: &str = "PIZZERIA_FRONTEND_BACKEND_PORT";
const BACKEND_HOST_DEFAULT: Ipv4Addr = Ipv4Addr::new(127, 0, 0, 1);
const BACKEND_PORT_DEFAULT: u16 = 3333;

pub fn parse_arguments() -> Result<SocketAddr, FrontendError> {
    match env::args_os().nth(1) {
        Some(os) => {
            let s = os.into_string().map_err(|_| FrontendError::NotUnicodeArg)?;
            s.parse::<SocketAddr>()
                .map_err(|error| FrontendError::InvalidSocketAddr { value: s, source: error})
        }
        None => Ok(SocketAddr::new(IpAddr::V4(BACKEND_HOST_DEFAULT), BACKEND_PORT_DEFAULT))
    }
}
