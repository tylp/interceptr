use std::{fmt::Display, process::Command};
use thiserror::Error;

#[derive(Debug, Error)]
pub enum IpTablesError {
    #[error("Rule not found: {0}")]
    RuleNotFound(String),
    #[error("Error adding rule: {0}")]
    AddRuleError(String),
}

#[derive(Debug, PartialEq, PartialOrd, Eq, Ord)]
pub enum Filter {
    Input,
    Output,
    Forward,
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

#[derive(Debug, PartialEq, PartialOrd, Eq, Ord)]
struct IpTableRule {
    filter: Filter,
    ip: String,
    queue: u16,
}

impl Drop for IpTableRule {
    fn drop(&mut self) {
        println!("Dropping rule: {:?}", self);
        let cmd = Command::new("sudo")
            .arg("iptables")
            .arg("-D")
            .arg(self.filter.to_string())
            .arg("-s")
            .arg(self.ip.clone())
            .arg("-j")
            .arg("NFQUEUE")
            .arg("--queue-num")
            .arg(self.queue.to_string())
            .status();

        match cmd {
            Ok(_) => (),
            Err(e) => eprintln!("Error dropping rule: {:?}", e),
        }
    }
}

pub struct IpTables {
    rules: Vec<IpTableRule>,
}

impl IpTables {
    /// Remove an active rule from the iptables.
    /// If the rule does not exist, nothing happens.
    pub fn remove_rule(
        &mut self,
        filter: Filter,
        ip: &str,
        queue: u16,
    ) -> Result<(), IpTablesError> {
        let rule = IpTableRule {
            filter,
            ip: ip.to_string(),
            queue,
        };

        if let Some(index) = self.rules.iter().position(|r| r == &rule) {
            self.rules.remove(index);
            return Ok(());
        }

        Err(IpTablesError::RuleNotFound(format!("{:?}", rule)))
    }

    /// Create a rule in the iptables for an ip address to redirect packets to the given queue.
    ///
    /// Example:
    ///
    /// ```rust
    /// // Add a rule to send all packets comming from (INPUT) 127.0.0.1 to the NFQUEUE 0.
    /// add_rule(Filter::INPUT, "127.0.0.1", 0).unwrap().
    /// ```
    pub fn add_rule(&mut self, filter: Filter, ip: &str, queue: u16) -> Result<(), IpTablesError> {
        println!("Running command 'iptables -I {filter} -s {ip} -j NFQUEUE --queue-num {queue}'");

        let cmd = Command::new("sudo")
            .arg("iptables")
            .arg("-A")
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
            Err(e) => return Err(IpTablesError::AddRuleError(e.to_string())),
        };

        if !status.success() {
            return Err(IpTablesError::AddRuleError(
                status.code().expect("no status code").to_string(),
            ));
        }

        self.rules.push(IpTableRule {
            filter,
            ip: ip.to_string(),
            queue,
        });

        Ok(())
    }
}
