use interceptr::nfq;
use interceptr::iptables::{add_queue_redirection, Filter};
#[tokio::main]
async fn main() {
    
    let queue_number = 1;
    let queue = nfq::create_queue(queue_number).unwrap();

    add_queue_redirection(Filter::Input, "127.0.0.1", queue_number).unwrap();

    let _task = nfq::listen_queue(queue).await;
}