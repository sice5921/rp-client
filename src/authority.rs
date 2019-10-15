use std::fmt;
use std::ops::Range;
use std::str::FromStr;

use crate::error::{Error, Result};
use crate::range::{get_chunks, RangeUsize};

#[derive(Clone, Debug, PartialEq)]
pub struct Authority {
    inner: String,
    username: Option<RangeUsize>,
    password: Option<RangeUsize>,
    host: RangeUsize,
    port: Option<RangeUsize>,
}

impl Authority {
    pub fn username(&self) -> Option<&str> {
        self.username.map(|r| &self.inner[r])
    }

    pub fn password(&self) -> Option<&str> {
        self.password.map(|r| &self.inner[r])
    }

    pub fn user_info(&self) -> Option<&str> {
        match (&self.username, &self.password) {
            (Some(u), Some(p)) => Some(&self.inner[u.start..p.end]),
            (Some(u), None) => Some(&self.inner[*u]),
            _ => None,
        }
    }

    pub fn host(&self) -> &str {
        &self.inner[self.host]
    }

    pub fn port(&self) -> Option<u16> {
        match &self.port {
            Some(p) => Some(self.inner[*p].parse().unwrap()),
            None => None,
        }
    }
}

impl FromStr for Authority {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self> {
        let inner = s.to_string();

        let mut username = None;
        let mut password = None;

        let uri_part = if s.contains('@') {
            let (info, part) = get_chunks(&s, Some(RangeUsize::new(0, s.len())), "@", true, false);
            let (name, pass) = get_chunks(&s, info, ":", true, false);

            username = name;
            password = pass;

            part
        } else {
            Some(RangeUsize::new(0, s.len()))
        };

        let split_by = if s.contains(']') && s.contains('[') {
            "]:"
        } else {
            ":"
        };
        let (host, port) = get_chunks(&s, uri_part, split_by, true, false);
        let host = host.ok_or(Error::ParseHost)?;

        if let Some(p) = port {
            if inner[p].parse::<u16>().is_err() {
                return Err(Error::ParsePort);
            }
        }

        Ok(Authority {
            inner,
            username,
            password,
            host,
            port,
        })
    }
}

impl fmt::Display for Authority {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let auth = if let Some(pass) = self.password {
            let range = Range::from(pass);

            let hidden_pass = "*".repeat(range.len());
            let mut auth = self.inner.to_string();
            auth.replace_range(range, &hidden_pass);

            auth
        } else {
            self.inner.to_string()
        };

        write!(f, "{}", auth)
    }
}