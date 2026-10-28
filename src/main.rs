use std::sync::Arc;
use std::time::Duration;
use tokio::sync::Mutex;
use tokio::net::TcpStream;
use tokio::io::AsyncReadExt;
use tokio::time::interval;
use communication_library_rs::frame::Frame;

use stats::{handle_frame, Stats};

mod stats;

#[tokio::main]
async fn main() {
    let stream = TcpStream::connect("127.0.0.1:3000").await.unwrap();
    let (mut reader, _writer) = stream.into_split();

    let stats = Arc::new(Mutex::new(Stats::default()));

    let stats_clone = Arc::clone(&stats);
    let read_task = tokio::spawn(async move {
        let mut buf = [0u8; 14];
        loop {
            reader.read_exact(&mut buf).await.unwrap();
            let frame = Frame::unpack(&buf).unwrap();

            handle_frame(frame, stats_clone.clone());
        }
    });
    let _display_task = tokio::spawn(async move {
        let mut ticker = interval(Duration::from_millis(250));

        loop {
            ticker.tick().await;
            let stats = stats.lock().await;
            println!("\n{}", stats);
        }
    });

    let _ = tokio::join!(read_task);
}
