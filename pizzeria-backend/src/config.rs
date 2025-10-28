use std::env;
use std::env::VarError;
use std::error::Error;
use std::net::{IpAddr, Ipv4Addr, SocketAddr};
use std::str::FromStr;
use crate::custom_error::ConfigError;

#[derive(Debug, PartialEq)]
pub struct Config {
    bind_host: IpAddr,
    bind_port: u16,
}

const BIND_HOST_KEY: &str = "PIZZERIA_BACKEND_BIND_HOST";
const BIND_HOST_DEFAULT: IpAddr = IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1));
const BIND_PORT_KEY: &str = "PIZZERIA_BACKEND_BIND_PORT";
const BIND_PORT_DEFAULT: u16 = 3333;

pub fn load_configuration_from_environment_variables() -> Result<Config, ConfigError> {
    Ok(Config {
        bind_host: extract_environment_variable(BIND_HOST_KEY, BIND_HOST_DEFAULT)?,
        bind_port: extract_environment_variable(BIND_PORT_KEY, BIND_PORT_DEFAULT)?
    })
}

fn extract_environment_variable<T>(key: &str, default: T) -> Result<T, ConfigError>
where T: FromStr,
      T::Err: Error + Send + Sync + 'static {

   match env::var(key) {
        Ok(value) => value.parse::<T>().map_err(|error| ConfigError::Parse {
            key: key.to_string(),
            value,
            source: Box::new(error),
        }),
        Err(VarError::NotPresent) => {
            Ok(default)
        }
        Err(error @ VarError::NotUnicode(_)) => {
            Err(ConfigError::NotUnicode { key: key.to_string(), source: error })
        }
    }
}

pub fn get_socket_address() -> Result<SocketAddr, ConfigError> {
    let configuration = load_configuration_from_environment_variables()?;
    Ok(SocketAddr::new(configuration.bind_host, configuration.bind_port))
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
