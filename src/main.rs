use std::thread;
use std::time;

#[derive(foxglove::Encode)]
struct Message {
    elapsed: f64,
    message: String
}

fn main() {
    foxglove::WebSocketServer::new()
        .start_blocking()
        .expect("Server failed to start");

    loop {
        let time = time::SystemTime::now();
        foxglove::log!(
            "/hello",
            Message {
                elapsed: time
                    .elapsed()
                    .expect("clock failed to elapse")
                    .as_secs_f64(),
                message: String::from("test")
            }
        );
        thread::sleep(time::Duration::from_millis(30));
    }
}
