use nfq::{Queue, Verdict};
use thiserror::Error;

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
pub fn create_queue(queue_num: u16) -> Result<(), CreateQueueError> {

    let mut queue = match Queue::open() {
        Ok(queue) => queue,
        Err(e) => return Err(CreateQueueError::OpenError(queue_num)),
    };

    if let Err(e) = queue.bind(queue_num) {
        return Err(CreateQueueError::BindError(queue_num));
    }

    // spawn a tokio task that will handle packets
    tokio::spawn(async move {
        loop {
            let mut msg = match queue.recv() {
                Ok(msg) => msg,
                Err(e) => {
                    eprintln!("{e}");
                    return;
                },
            };

            msg.set_verdict(Verdict::Accept);
            if let Err(e) = queue.verdict(msg) {
                eprintln!("{e}");
                return;
            }
        }
    });

    Ok(())
}