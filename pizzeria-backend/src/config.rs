use std::env;
use std::env::VarError;
use std::net::IpAddr;
use std::str::FromStr;

#[derive(Debug, PartialEq)]
pub struct Config {
    bind_host: IpAddr,
    bind_port: u16,
}

const BIND_HOST_KEY: &str = "PIZZERIA_BACKEND_BIND_HOST";
const BIND_PORT_KEY: &str = "PIZZERIA_BACKEND_BIND_PORT";

pub fn load_configuration_from_environment_variables() -> Result<Config, Box<dyn std::error::Error>> {
    let bind_host = env::var(BIND_HOST_KEY);

    let bind_host =  match bind_host {
        Ok(bind_host) => IpAddr::from_str(&bind_host)?, //todo: add error context
        Err(VarError::NotPresent) => {
            IpAddr::from_str("127.0.0.1")
                .expect("Should always be able to parse default IP address")
        }
        Err(error @ VarError::NotUnicode(_)) => {
            return Err(Box::new(error));
        }
    };

    let bind_port = env::var(BIND_PORT_KEY);

    let bind_port = match bind_port {
        Ok(bind_port) => u16::from_str(&bind_port)?,    //todo: add error context
        Err(VarError::NotPresent) => {
            3333
        }
        Err(error @ VarError::NotUnicode(_)) => {
            return Err(Box::new(error))
        }
    };

    Ok(Config {
        bind_host,
        bind_port,
    })
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
