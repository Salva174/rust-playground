use std::env::VarError;
use std::error::Error;
use std::{fmt, io};
use std::fmt::{Display, Formatter};
use std::io::ErrorKind::{InvalidData, InvalidInput, Other};
use std::net::AddrParseError;
use std::num::ParseIntError;
use std::string::FromUtf8Error;

#[derive(Debug)]
pub enum FrontendError {
    InvalidHost {
        key: &'static str,
        value: String,
        source: AddrParseError
    },
    InvalidPort {
        key: &'static str,
        value: String,
        source: ParseIntError
    },
    InvalidSocketAddr {
        value: String, source: AddrParseError
    },
    NotUnicode {
        key: &'static str,
        source: VarError,
    },
    NotUnicodeArg,
    HttpStatus {
        code: u16,
    },
    InvalidContentLength {
        value: String,
        source: ParseIntError
    },
    BodyUtf8 {
        source: FromUtf8Error
    },
    UnexpectedEof,
}

impl Display for FrontendError {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            FrontendError::InvalidHost { key, value, .. } =>
                write!(f, "Ungültiger Host in {key}: '{value}'"),
            FrontendError::InvalidPort { key, value, .. } =>
                write!(f, "Ungültiger Port in {key}: '{value}'"),
            FrontendError::InvalidSocketAddr { value,  .. } =>
                write!(f, "Ungültige Adresse '{value}'."),
            FrontendError::NotUnicode { key, ..} =>
                write!(f, "{key} ist nicht gültiges Unicode."),
            FrontendError::NotUnicodeArg =>
                write!(f, "Nicht gültiges Unicode."),
            FrontendError::HttpStatus { code} =>
                write!(f, "Backend antwortet mit HTTP {code}."),
            FrontendError::InvalidContentLength { value, ..} =>
                write!(f, "Ungültige Länge '{value}'."),
            FrontendError::BodyUtf8 { .. } =>
                write!(f, "Nicht gültiges UTF8."),
            FrontendError::UnexpectedEof =>
                write!(f, "Unerwartes Ende der Verbindung - Antwort unvollständig.")
        }
    }
}

impl Error for FrontendError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            FrontendError::InvalidHost { source, .. } => Some(source),
            FrontendError::InvalidPort { source, .. } => Some(source),
            FrontendError::InvalidSocketAddr { source, .. } => Some(source),
            FrontendError::NotUnicode { source, .. } => Some(source),
            FrontendError::NotUnicodeArg => None,
            FrontendError::HttpStatus { .. } => None,
            FrontendError::InvalidContentLength { source , .. } => Some(source),
            FrontendError::BodyUtf8 { source, .. } => Some(source),
            FrontendError::UnexpectedEof => None,
        }
    }
}

impl FrontendError {
    pub fn into_io(self) -> io::Error {
        let kind = match self {
            FrontendError::InvalidHost { .. } => InvalidInput,
            FrontendError::InvalidPort { .. } => InvalidInput,
            FrontendError::InvalidSocketAddr { .. } => InvalidInput,
            FrontendError::NotUnicode { .. } => InvalidInput,
            FrontendError::NotUnicodeArg  => InvalidInput,
            FrontendError::HttpStatus { .. } => Other,
            FrontendError::InvalidContentLength { .. } => InvalidData,
            FrontendError::BodyUtf8 { .. } => InvalidData,
            FrontendError::UnexpectedEof  => InvalidData,
        };
        io::Error::new(kind, self)
    }
}

#[cfg(test)]
mod tests {
    use std::env;
    use std::net::IpAddr;
    use serial_test::serial;
    use crate::{parse_arguments, BACKEND_HOST_KEY, BACKEND_PORT_KEY};
    use super::*;

    #[test]
    #[serial]
    fn invalid_host_is_caught() {

        let prev_host = env::var_os(BACKEND_HOST_KEY);
        let prev_port = env::var_os(BACKEND_PORT_KEY);

        unsafe { env::set_var(BACKEND_HOST_KEY, "not-an-ip"); }
        unsafe { env::remove_var(BACKEND_PORT_KEY); }

        let err = parse_arguments().unwrap_err();

        match err {
            FrontendError::InvalidHost { key, value, .. } => {
                assert_eq!(key, BACKEND_HOST_KEY);
                assert_eq!(value, "not-an-ip");
            }
            other => panic!("unexpected error: {other}"),
        }

        unsafe {
            match prev_host {
                Some(v) => env::set_var(BACKEND_HOST_KEY, v),
                None => env::remove_var(BACKEND_HOST_KEY)
            }
        }

        unsafe {
            match prev_port {
                Some(v) => env::set_var(BACKEND_PORT_KEY, v),
                None => env::remove_var(BACKEND_PORT_KEY)
            }
        }
    }

    #[test]
    fn invalid_host_message() {
        let bad = "not-an-ip".parse::<IpAddr>().unwrap_err();
        let err = FrontendError::InvalidHost {
            key: BACKEND_HOST_KEY,
            value: "not-an-ip".into(),
            source: bad,
        };
        assert_eq!(
            err.to_string(),
            format!("Ungültiger Host in {}: 'not-an-ip'", BACKEND_HOST_KEY)
        );

        // assert_eq!(err.to_string(), "...");
    }

    #[test]
    fn invalid_port_message() {
        let bad: ParseIntError = "abc".parse::<u16>().unwrap_err();
        let err = FrontendError::InvalidPort {
            key: BACKEND_PORT_KEY,
            value: "abc".into(),
            source: bad,
        };
        assert_eq!(
            err.to_string(),
            format!("Ungültiger Port in {}: 'abc'", BACKEND_PORT_KEY)
        );

        // assert_eq!(err.to_string(), "...");
    }

    #[test]
    fn http_status_message() {
        let err = FrontendError::HttpStatus { code: 503 };
        assert_eq!(err.to_string(), "Backend antwortet mit HTTP 503.");
    }

    #[test]
    fn invalid_content_length_message() {
        let src = "erwartung".parse::<usize>().unwrap_err();
        let err = FrontendError::InvalidContentLength {
            value: "erwartung".into(),
            source: src,
        };
        assert_eq!(err.to_string(), "Ungültige Länge 'erwartung'.");
    }

    #[test]
    fn body_utf8_message() {
        let src = String::from_utf8(vec![0xFF, 0xFF]).unwrap_err();
        let err = FrontendError::BodyUtf8 { source: src };
        assert_eq!(err.to_string(), "Nicht gültiges UTF8.")
    }

    #[test]
    fn unexpected_eof_message() {
        let err = FrontendError::UnexpectedEof;
        assert_eq!(
            err.to_string(),
            "Unerwartetes Ende der Verbindung - Antwort unvollständig."
        );
    }
}
