use interceptr::nfq;

#[tokio::main]
async fn main() {
    let queue_number = 1;
    let queue = nfq::create_queue(queue_number).unwrap();
    let _task = nfq::listen_queue(queue).await;
}
