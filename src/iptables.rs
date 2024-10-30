use anyhow::{anyhow, Result};
use std::process::Command;

pub enum Filter {
    Input,
    Output,
    Forward,
}

pub fn add_rule(filter: Filter, ip: &str, queue_num: u16) -> Result<()> {
    // sudo iptables -I INPUT -s 192.168.1.100 -j NFQUEUE --queue-num 0
    let filter_str = match filter {
        Filter::Input => "INPUT",
        Filter::Output => "OUTPUT",
        Filter::Forward => "FORWARD",
    };

    let cmd = Command::new("iptables")
        .arg("-I")
        .arg(filter_str)
        .arg("-s")
        .arg(ip)
        .arg("-j")
        .arg("NFQUEUE")
        .arg("--queue-num")
        .arg(queue_num.to_string())
        .status();

    match cmd {
        Ok(status) => {
            if status.success() {
                Ok(())
            } else {
                Err(anyhow!("Failed to add iptables rule"))
            }
        }
        Err(e) => Err(anyhow!(e)),
    }
}
