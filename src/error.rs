use std::{error, fmt, io, net, result};

pub type Result<T> = result::Result<T, Error>;

#[derive(Debug)]
pub enum Error {
    EmptyScheme,
    EmptyAuthority,
    Io(io::Error),
    HandshakeError(native_tls::HandshakeError<std::net::TcpStream>),
    StdParseAddr(net::AddrParseError),
    NoneString,
    ParseFragment(&'static str),
    ParseHost,
    ParseAddr,
    ParseHeaders,
    ParseIPv6,
    ParsePort,
    ParseQuery(&'static str),
    ParseScheme,
    ParseUserInfo(&'static str),
    NativeTls(native_tls::Error),
    UnknownMethod(String),
    UnsupportedProxyScheme,
    UnsupportedScheme(String),
    UnsupportedVersion(String),
    WrongHttp,
    InvalidServerVersion,
    InvalidAuthVersion,
    AuthFailure,
    InvalidAuthMethod,
    InvalidAddressType,
    InvalidReservedByte,
    UnknownError,
    InvalidCommandProtocol,
    TtlExpired,
    RefusedByHost,
    HostUnreachable,
    NetworkUnreachable,
    InvalidRuleset,
    GeneralFailure,
}

impl fmt::Display for Error {
    fn fmt(&self, w: &mut fmt::Formatter) -> fmt::Result {
        use self::Error::*;

        match self {
            EmptyScheme => write!(w, "Uri no have scheme"),
            EmptyAuthority => write!(w, "Uri no have authority"),
            Io(e) => write!(w, "{}", e),
            HandshakeError(e) => write!(w, "{}", e),
            StdParseAddr(e) => write!(w, "{}", e),
            NoneString => write!(w, "none string"),
            ParseFragment(e) => write!(w, "parse fragmeng {}", e),
            ParseHost => write!(w, "parse host"),
            ParseAddr => write!(w, "parse addr"),
            ParseHeaders => write!(w, "parse headers"),
            ParseIPv6 => write!(w, "parse ip version 6"),
            ParsePort => write!(w, "parse port"),
            ParseQuery(e) => write!(w, "parse query {}", e),
            ParseScheme => write!(w, "parse scheme"),
            ParseUserInfo(e) => write!(w, "parse user info {}", e),
            NativeTls(e) => write!(w, "{}", e),
            UnknownMethod(e) => write!(w, "unknown method {}", e),
            UnsupportedProxyScheme => write!(w, "unsupported proxy scheme"),
            UnsupportedScheme(e) => write!(w, "unsupported scheme {}", e),
            UnsupportedVersion(e) => write!(w, "unsupported version {}", e),
            WrongHttp => write!(w, "wrong http"),
            InvalidServerVersion => write!(w, "invalid socks server version"),
            InvalidAuthVersion => write!(w, "invalid auth version"),
            AuthFailure => write!(w, "failure, connection must be closed"),
            InvalidAuthMethod => write!(w, "auth method not supported"),
            InvalidAddressType => write!(w, "Invalid address type"),
            InvalidReservedByte => write!(w, "Invalid reserved byte"),
            UnknownError => write!(w, "Unknown error"),
            InvalidCommandProtocol => write!(w, "Command not supported / protocol error"),
            TtlExpired => write!(w, "TTL expired"),
            RefusedByHost => write!(w, "Connection refused by destination host"),
            HostUnreachable => write!(w, "Host unreachable"),
            NetworkUnreachable => write!(w, "Network unreachable"),
            InvalidRuleset => write!(w, "Connection not allowed by ruleset"),
            GeneralFailure => write!(w, "General failure"),
        }
    }
}

impl error::Error for Error {
    fn description(&self) -> &str {
        use self::Error::*;

        match self {
            EmptyScheme => "Uri no have scheme",
            EmptyAuthority => "Uri no have authority",
            Io(e) => e.description(),
            HandshakeError(e) => e.description(),
            StdParseAddr(e) => e.description(),
            NoneString => "none string",
            ParseFragment(_) => "parse fragmeng",
            ParseHost => "parse host",
            ParseAddr => "parse addr",
            ParseHeaders => "parse headers",
            ParseIPv6 => "parse ip version 6",
            ParsePort => "parse port",
            ParseQuery(_) => "parse query",
            ParseScheme => "parse scheme",
            ParseUserInfo(_) => "parse user info",
            NativeTls(e) => e.description(),
            UnknownMethod(_) => "unknown method",
            UnsupportedProxyScheme => "unsupported proxy scheme",
            UnsupportedScheme(_) => "unsupported scheme",
            UnsupportedVersion(_) => "unsupported version",
            WrongHttp => "wrong http",
            InvalidServerVersion => "invalid socks server version",
            InvalidAuthVersion => "invalid auth version",
            AuthFailure => "failure, connection must be closed",
            InvalidAuthMethod => "auth method not supported",
            InvalidAddressType => "Invalid address type",
            InvalidReservedByte => "Invalid reserved byte",
            UnknownError => "Unknown error",
            InvalidCommandProtocol => "Command not supported / protocol error",
            TtlExpired => "TTL expired",
            RefusedByHost => "Connection refused by destination host",
            HostUnreachable => "Host unreachable",
            NetworkUnreachable => "Network unreachable",
            InvalidRuleset => "Connection not allowed by ruleset",
            GeneralFailure => "General failure",
        }
    }

    fn source(&self) -> Option<&(dyn error::Error + 'static)> {
        use self::Error::*;

        match self {
            EmptyScheme => None,
            EmptyAuthority => None,
            Io(e) => e.source(),
            HandshakeError(e) => e.source(),
            StdParseAddr(e) => e.source(),
            NoneString => None,
            ParseFragment(_) => None,
            ParseHost => None,
            ParseAddr => None,
            ParseHeaders => None,
            ParseIPv6 => None,
            ParsePort => None,
            ParseQuery(_) => None,
            ParseScheme => None,
            ParseUserInfo(_) => None,
            NativeTls(e) => e.source(),
            UnknownMethod(_) => None,
            UnsupportedProxyScheme => None,
            UnsupportedScheme(_) => None,
            UnsupportedVersion(_) => None,
            WrongHttp => None,
            InvalidServerVersion => None,
            InvalidAuthVersion => None,
            AuthFailure => None,
            InvalidAuthMethod => None,
            InvalidAddressType => None,
            InvalidReservedByte => None,
            UnknownError => None,
            InvalidCommandProtocol => None,
            TtlExpired => None,
            RefusedByHost => None,
            HostUnreachable => None,
            NetworkUnreachable => None,
            InvalidRuleset => None,
            GeneralFailure => None,
        }
    }
}

impl From<io::Error> for Error {
    fn from(err: io::Error) -> Error {
        Error::Io(err)
    }
}

impl From<net::AddrParseError> for Error {
    fn from(err: net::AddrParseError) -> Error {
        Error::StdParseAddr(err)
    }
}

impl From<native_tls::Error> for Error {
    fn from(err: native_tls::Error) -> Error {
        Error::NativeTls(err)
    }
}

impl From<native_tls::HandshakeError<std::net::TcpStream>> for Error {
    fn from(err: native_tls::HandshakeError<std::net::TcpStream>) -> Error {
        Error::HandshakeError(err)
    }
}
