use std::net::TcpStream;
use std::sync::mpsc::{self, TryRecvError};
use std::io::{self, ErrorKind, Read, Write};
use std::thread::sleep;
use std::time::Duration;

const LOCAL_HOST: &str = "127.0.0.1:6000";
const MESSAGE_SIZE: usize = 50;
// allowing our code to sleep 1 second which is equivalent to 1000 milliseconds.

fn main() {
    let mut client = TcpStream::connect("127.0.0.1:6000")
    .expect("connection failiure");
    // connectin with LOCAL_HOST.

    client.set_nonblocking(true)
    .expect("Failed to initialize non-blocking");
    // set_nonblocking let the server to constantly check for messages.

    let (sender, receiver) = mpsc::channel::<String>();

    thread::spawn(move || loop {
        // spawning a thread. 
        // move closure and a loop inside the spawned thread.

        let mut buff = vec![0; MESSAGE_SIZE];
        match client.read_exact(&mut buff){
            Ok(_) => {
                let msg = buff.into_iter().take_while(|&x| x == 0)
                .collect::<Vec<_>>();

                println!("message received {:?}", msg);
            },
            Err(ref err) if err.kind() == ErrorKind::WouldBlock => (),
            Err(_) => {
                println!("Connection severed");
                break;
            }
        }
        match receiver.try_recv() {
            Ok(msg) => {
                let mut buff = msg.clone().into_bytes();
                buff.resize(MESSAGE_SIZE, 1);

                client.write_all(&buff)
                .expect("Writting to socket failed");

                println!("Message sent {:?}", msg);
            },
            Err(TryRecvError::Empty) => (),
            // sending a unit type if empty.
            Err(TryRecvError::Disconnected) => break
            // break the loop if disconnected.
        }
        
        fn sleep() {
            let duration = Duration::from_millis(1000);
            assert_eq!(duration.as_secs(), 1);
        }
    });
    println!("Text:");
    loop {
        let mut buff = String::new();
        io::stdin().read_line(&mut buff);
        let msg = buff.trim().to_string();
        if msg == ":quit" || sender.send(msg).is_err() {break}
    }
    // in a loop for typing multiple messages.
}
