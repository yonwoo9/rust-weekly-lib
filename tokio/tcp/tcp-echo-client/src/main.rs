use core::time;
use std::thread;

use tokio::{
    io::{self, AsyncReadExt, AsyncWriteExt},
    net::TcpStream,
};

#[tokio::main]
async fn main() {
    let socket = TcpStream::connect("127.0.0.1:8080").await.unwrap();
    let (mut rd, mut wr) = io::split(socket);
    let mut buf = vec![0; 1024];

    tokio::spawn(async move {
        loop {
            wr.write_all(b"test").await.unwrap();
            thread::sleep(time::Duration::from_secs(1));
        }
    });
    loop {
        match rd.read(&mut buf).await {
            Ok(n) => {
                if n == 0 {
                    break;
                } else {
                    println!("client get buf:{:?}", std::str::from_utf8(&buf[..n]));
                }
            }

            Err(e) => {
                if e.kind() == io::ErrorKind::WouldBlock {
                    continue;
                } else {
                    eprintln!("read buf err:{:?}", e)
                }
            }
        }
    }
}
