#![allow(missing_docs)]

use std::fmt;
use std::str::FromStr;

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct Url {
    pub(crate) inner: url::Url,
}

impl Url {
    pub fn parse(input: &str) -> Result<Self, url::ParseError> {
        Ok(Url {
            inner: url::Url::parse(input)?,
        })
    }

    pub fn as_str(&self) -> &str {
        self.inner.as_str()
    }

    pub fn scheme(&self) -> &str {
        self.inner.scheme()
    }

    pub fn has_authority(&self) -> bool {
        self.inner.has_authority()
    }

    pub fn has_host(&self) -> bool {
        self.inner.has_host()
    }

    pub fn host_str(&self) -> Option<&str> {
        self.inner.host_str()
    }

    pub fn port(&self) -> Option<u16> {
        self.inner.port()
    }

    pub fn port_or_known_default(&self) -> Option<u16> {
        self.inner.port_or_known_default()
    }

    pub fn path(&self) -> &str {
        self.inner.path()
    }

    pub fn query(&self) -> Option<&str> {
        self.inner.query()
    }

    pub fn username(&self) -> &str {
        self.inner.username()
    }

    pub fn password(&self) -> Option<&str> {
        self.inner.password()
    }

    pub fn set_username(&mut self, username: &str) -> Result<(), ()> {
        self.inner.set_username(username)
    }

    pub fn set_password(&mut self, password: Option<&str>) -> Result<(), ()> {
        self.inner.set_password(password)
    }

    pub fn set_port(&mut self, port: Option<u16>) -> Result<(), ()> {
        self.inner.set_port(port)
    }

    pub fn set_query(&mut self, query: Option<&str>) {
        self.inner.set_query(query)
    }

    pub fn set_fragment(&mut self, fragment: Option<&str>) {
        self.inner.set_fragment(fragment)
    }

    pub fn query_pairs_mut(&mut self) -> url::form_urlencoded::Serializer<'_, url::UrlQuery<'_>> {
        self.inner.query_pairs_mut()
    }
}

impl fmt::Display for Url {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        fmt::Display::fmt(&self.inner, f)
    }
}

impl FromStr for Url {
    type Err = url::ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Self::parse(s)
    }
}

impl AsRef<str> for Url {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl From<Url> for String {
    fn from(shim: Url) -> String {
        shim.inner.to_string()
    }
}

impl TryFrom<&str> for Url {
    type Error = url::ParseError;

    fn try_from(s: &str) -> Result<Self, Self::Error> {
        Self::parse(s)
    }
}
