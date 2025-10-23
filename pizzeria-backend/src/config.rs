use std::env;
use std::env::VarError;
use std::fmt::Display;
use std::net::{IpAddr, Ipv4Addr};
use std::str::FromStr;

#[derive(Debug, PartialEq)]
pub struct Config {
    bind_host: IpAddr,
    bind_port: u16,
}

const BIND_HOST_KEY: &str = "PIZZERIA_BACKEND_BIND_HOST";
const BIND_HOST_DEFAULT: IpAddr = IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1));
const BIND_PORT_KEY: &str = "PIZZERIA_BACKEND_BIND_PORT";
const BIND_PORT_DEFAULT: u16 = 3333;

pub fn load_configuration_from_environment_variables() -> Result<Config, Box<dyn std::error::Error>> {
    Ok(Config {
        bind_host: extract_environment_variable(BIND_HOST_KEY, BIND_HOST_DEFAULT)?,
        bind_port: extract_environment_variable(BIND_PORT_KEY, BIND_PORT_DEFAULT)?
    })
}

fn extract_environment_variable<T: FromStr>(key: &str, default: T) -> Result<T, Box<dyn std::error::Error>>
where <T as FromStr>::Err: std::error::Error + 'static {
    let value = env::var(key);

    let value =  match value {
        Ok(value) => T::from_str(&value)?, //todo: add error context
        Err(VarError::NotPresent) => {
            default
        }
        Err(error @ VarError::NotUnicode(_)) => {
            return Err(Box::new(error));
        }
    };
    Ok(value)
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_load_configuration_from_environment_variables() {

        let bind_host = IpAddr::from_str("0.0.0.0").unwrap();
        let bind_port = 1234;

        unsafe {
            env::set_var(BIND_HOST_KEY, bind_host.to_string());
            env::set_var(BIND_PORT_KEY, bind_port.to_string());
        }

        let result = load_configuration_from_environment_variables().unwrap();

        assert_eq!(result, Config {
            bind_host,
            bind_port,
        });
    }
}
