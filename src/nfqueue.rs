use nfq::{Queue, Verdict};

/**
 * TODO: The program must have the right capabilities: sudo setcap 'cap_net_admin=+ep' /home/apl/dev/interceptr/target/debug/interceptr
 *
 * Create a new queue in the iptables.
 * - sudo iptables -I INPUT -s 192.168.1.100 -j NFQUEUE --queue-num 0
 * - sudo iptables -I OUTPUT -s 192.168.1.100 -j NFQUEUE --queue-num 0
 * - sudo iptables -I FORWARD -s 192.168.1.100 -j NFQUEUE --queue-num 0
 */

fn init() -> std::io::Result<()> {
    // iptables::add_rule(iptables::Filter::Forward, "127.0.0.1", 0).unwrap();

    let mut queue = Queue::open()?;
    queue.bind(0)?;
    loop {
        let mut msg = queue.recv()?;
        msg.set_verdict(Verdict::Accept);
        queue.verdict(msg)?;
    }
    Ok(())
}
