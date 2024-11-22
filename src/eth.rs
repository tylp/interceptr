use std::{collections::HashMap, process::Command};

use thiserror::Error;

#[derive(Debug, PartialEq, PartialOrd, Eq, Ord, Hash)]
pub struct Eth(String);

#[derive(Debug, PartialEq, PartialOrd, Eq, Ord, Hash)]
pub struct Ip(String);

#[derive(Debug, Error)]
pub enum EthError {
    #[error("Error adding IP to eth: {0} does not exists")]
    EthNotFoundError(String),
    #[error("Error adding IP to eth: {0}")]
    IOError(String),
}

#[derive(Debug, Default)]
pub struct EthHandler {
    map: HashMap<Eth, Vec<Ip>>,
}

impl EthHandler {
    /// Adds an ip to the given network interface.
    pub fn add_ip(&mut self, ip: &str, link: &str) -> Result<(), EthError> {
        let cmd = Command::new("ip")
            .arg("addr")
            .arg("add")
            .arg(ip)
            .arg("dev")
            .arg(link)
            .status();

        let status = match cmd {
            Ok(status) => status,
            Err(e) => return Err(EthError::IOError(e.to_string())),
        };

        if !status.success() {
            return Err(EthError::IOError(
                status.code().expect("no status code").to_string(),
            ));
        }

        let ip = Ip(ip.to_string());
        let eth = Eth(link.to_string());

        if let Some(eth) = self.map.get_mut(&eth) {
            eth.push(ip);
        } else {
            return Err(EthError::EthNotFoundError(link.to_string()));
        }

        Ok(())
    }

    /// Deletes an ip from the given network interface.
    pub fn del_ip(&mut self, ip: &str, link: &str) -> Result<(), EthError> {
        let cmd = Command::new("ip")
            .arg("addr")
            .arg("add")
            .arg(ip)
            .arg("dev")
            .arg(link)
            .status();

        let status = match cmd {
            Ok(status) => status,
            Err(e) => return Err(EthError::IOError(e.to_string())),
        };

        if !status.success() {
            return Err(EthError::IOError(
                status.code().expect("no status code").to_string(),
            ));
        }

        let ip = Ip(ip.to_string());
        let eth = Eth(link.to_string());

        if let Some(eth) = self.map.get_mut(&eth) {
            if let Some(index) = eth.iter().position(|i| i == &ip) {
                eth.remove(index);
            }
        } else {
            return Err(EthError::EthNotFoundError(link.to_string()));
        }

        Ok(())
    }
}
