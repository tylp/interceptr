use nfq::{Queue, Verdict};
use thiserror::Error;
use tokio::task::JoinHandle;

#[derive(Debug, Error)]
pub enum CreateQueueError {
    #[error("Error while trying to open the queue {0}")]
    OpenError(u16),
    #[error("Error while trying to bind the queue {0}")]
    BindError(u16)
}

/// Create and binds an nfqueue with a given number.
/// Listen and apply defined rules to the incomming messages on this queue
/// trough a tokio task.
pub fn create_queue(queue_num: u16) -> Result<Queue, CreateQueueError> {

    println!("Creating queue with number {queue_num}");
    let mut queue = match Queue::open() {
        Ok(queue) => queue,
        Err(e) => {
            eprintln!("{e}");
            return Err(CreateQueueError::OpenError(queue_num))
        }
    };

    if let Err(e) = queue.bind(queue_num) {
        eprintln!("{e}");
        return Err(CreateQueueError::BindError(queue_num));
    }

    Ok(queue)
}

pub fn listen_queue(mut queue: Queue) -> JoinHandle<()> {
    tokio::spawn(async move {
        println!("Listening for messages on queue");
        loop {
            let mut msg = match queue.recv() {
                Ok(msg) => msg,
                Err(e) => {
                    eprintln!("{e}");
                    return;
                },
            };

            let payload = msg.get_payload();
            let queue_num = msg.get_queue_num();
            let timestamp = msg.get_timestamp();
            let packet_id = msg.get_packet_id();

            println!("Received packet: 
            id: {},
            queue_num: {},
            payload: {:02X?}", packet_id, queue_num, payload);

            msg.set_verdict(Verdict::Accept);
            if let Err(e) = queue.verdict(msg) {
                eprintln!("{e}");
                return;
            }
        }
    })
}