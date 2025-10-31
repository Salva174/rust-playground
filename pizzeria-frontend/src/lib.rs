use std::env;
use std::io::Write;
use std::io::Stdout;
use std::net::{IpAddr, Ipv4Addr, SocketAddr};
use crate::error::FrontendError;

pub mod input;
pub mod render;
pub mod state;
pub mod update;
pub mod custom_toppings;
pub mod http;
mod ui;
mod transactions;
pub mod toppings;
pub mod table;
pub mod table_menu;
pub mod types;
mod error;

pub fn clear_screen(stdout: &mut Stdout) -> Result<(), Box<dyn std::error::Error>> {
    write!(stdout, "\x1B[2J\x1B[1;1H")?;
    stdout.flush()?;
    Ok(())
}

pub const BACKEND_HOST_KEY: &str = "PIZZERIA_FRONTEND_BACKEND_HOST";
pub const BACKEND_PORT_KEY: &str = "PIZZERIA_FRONTEND_BACKEND_PORT";
const BACKEND_HOST_DEFAULT: Ipv4Addr = Ipv4Addr::new(127, 0, 0, 1);
const BACKEND_PORT_DEFAULT: u16 = 3333;

#[derive(Debug)]
pub struct Arguments {
    pub server_address: SocketAddr,
}

pub fn parse_arguments() -> Result<Arguments, FrontendError> {
    let server_address = match env::args_os().nth(1) {
        Some(os) => {
            let s = os.into_string().map_err(|_| FrontendError::NotUnicodeArg)?;
            s.parse::<SocketAddr>()
                .map_err(|error| FrontendError::InvalidSocketAddr { value: s, source: error})
        }
        None => Ok(SocketAddr::new(IpAddr::V4(BACKEND_HOST_DEFAULT), BACKEND_PORT_DEFAULT))
    }?;
    Ok(Arguments {
        server_address
    })
}
