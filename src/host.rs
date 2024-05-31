use std::path::{Path, PathBuf};

#[derive(Debug, Clone)]
pub struct Hostname(String);
#[derive(Debug, Clone)]
pub struct Port(u16);
#[derive(Debug, Clone)]
pub struct Username(String);
#[derive(Debug, Clone)]
pub struct Password(String);

impl Hostname {
    pub fn new(value: &str) -> Self {
        if value.is_empty() {
            panic!("Hostname cannot be empty");
        }
        Hostname(value.to_string())
    }
}
impl Into<String> for Hostname {
    fn into(self) -> String {
        self.0
    }
}

impl Port {
    pub fn new(value: u16) -> Self {
        if value == 0 {
            panic!("Port number cannot be 0");
        }
        Port(value)
    }
}
impl Into<u16> for Port {
    fn into(self) -> u16 {
        self.0
    }
}

impl Username {
    pub fn new(value: &str) -> Self {
        if value.is_empty() {
            panic!("Username cannot be empty");
        }
        Username(value.to_string())
    }
}
impl Into<String> for Username {
    fn into(self) -> String {
        self.0
    }
}

impl Password {
    pub fn new(value: &str) -> Self {
        if value.is_empty() {
            panic!("Password cannot be empty");
        }
        Password(value.to_string())
    }
}
impl Into<String> for Password {
    fn into(self) -> String {
        self.0
    }
}

#[derive(Debug)]
pub struct HostWithPassword {
    pub hostname: Hostname,
    pub port: Port,
    pub username: Username,
    pub password: Password,
}

impl HostWithPassword {
    pub fn new(hostname: &str, port: u16, username: &str, password: &str) -> Self {
        HostWithPassword {
            hostname: Hostname::new(hostname),
            port: Port::new(port),
            username: Username::new(username),
            password: Password::new(password),
        }
    }

    pub fn addrs(&self) -> (String, u16) {
        (self.hostname.clone().into(), self.port.clone().into())
    }
}

#[derive(Debug)]
pub struct HostWithKey {
    pub hostname: Hostname,
    pub port: Port,
    pub username: Username,
    pub key: PathBuf,
}

impl HostWithKey {
    pub fn new(hostname: &str, port: u16, username: &str, key: &Path) -> Self {
        HostWithKey {
            hostname: Hostname::new(hostname),
            port: Port::new(port),
            username: Username::new(username),
            key: key.to_path_buf(),
        }
    }

    pub fn addrs(&self) -> (String, u16) {
        (self.hostname.clone().into(), self.port.clone().into())
    }
}
