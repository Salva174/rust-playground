use std::env::VarError;
use std::error::Error;
use std::fmt::{Display, Formatter};
use std::fmt;

#[derive(Debug)]
pub enum ConfigError {
    NotUnicode {
        key: String,
        source: VarError,
    },
    Parse {
        key: String,
        value: String,
        source: Box<dyn Error + Send + Sync + 'static>
    },
}

impl Display for ConfigError {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            ConfigError::NotUnicode { key, .. } =>
                write!(f, "ENV {key} ist nicht gültiges Unicode"),
            ConfigError::Parse { key, value, source } =>
                write!(f, "ENV {key}='{value}' konnte nicht geparst werden: {source}")
        }
    }
}

impl Error for ConfigError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            ConfigError::NotUnicode { source, .. } => Some(source),
            ConfigError::Parse { source, ..} => Some(source.as_ref()),
        }
    }
}
