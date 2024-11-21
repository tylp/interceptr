use std::{fmt::Display, process::Command};
use thiserror::Error;

#[derive(Debug)]
pub enum Filter {
    Input,
    Output,
    Forward
}

#[derive(Debug, Error)]
pub enum CommandError {
    #[error("IO error")]
    IOError(#[from] std::io::Error),
    #[error("Error with status code `{0}`")]
    StatusError(i32),
}

impl Display for Filter {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let res = match self {
            Filter::Input => "INPUT",
            Filter::Output => "OUTPUT",
            Filter::Forward => "FORWARD",
        };

        write!(f, "{}", res)
    }
}

pub fn del_ip(ip: &str, link: &str) {
    let cmd = Command::new("ip")
        .arg("addr")
        .arg("add")
        .arg(ip)
        .arg("dev")
        .arg(link);
}

pub fn add_ip(ip: &str, link: &str) -> Result<(), CommandError> {
    let cmd = Command::new("ip")
        .arg("addr")
        .arg("add")
        .arg(ip)
        .arg("dev")
        .arg(link)
        .status();

        let status = match cmd {
            Ok(status) => status,
            Err(e) => return Err(CommandError::IOError(e))
        };


        match status.success() {
            true => return Ok(()),
            false => return Err(CommandError::StatusError(status.code().expect("no status code"))),
        }
}

/// Create a rule in the iptables for an ip address to redirect packets to the given queue.
/// 
/// Example:
/// 
/// ```rust
/// // Add a rule to send all packets comming from (INPUT) 127.0.0.1 to the NFQUEUE 0.
/// add_queue_redirection(Filter::INPUT, "127.0.0.1", 0).unwrap().
/// ```
pub fn add_queue_redirection(filter: Filter, ip: &str, queue: u16) -> Result<(), CommandError> {

    println!("Running command 'iptables -I {filter} -s {ip} -j NFQUEUE --queue-num {queue}'");
    println!("Queue num: {}", queue.to_string());


    let cmd = Command::new("sudo")
        .arg("iptables")
        .arg("-I")
        .arg(filter.to_string())
        .arg("-s")
        .arg(ip)
        .arg("-j")
        .arg("NFQUEUE")
        .arg("--queue-num")
        .arg(queue.to_string())
        .status();
    
    let status = match cmd {
        Ok(status) => status,
        Err(e) =>  {
            return Err(CommandError::IOError(e))
        }
    };

    match status.success() {
        true => return Ok(()),
        false => return Err(CommandError::StatusError(status.code().expect("no status code"))),
    }
}