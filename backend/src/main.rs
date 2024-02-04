use std::net::TcpListener;
use std::sync::{mpsc, Arc, RwLock};
use std::thread::spawn;
use tungstenite::accept;

struct Value {
    value: i32,
    b: i32,
}

struct ValueSet {
    id: usize,
    value: i32,
}

fn value_manager_setup() -> (mpsc::Sender<ValueSet>, Arc<RwLock<Vec<i32>>>) {
    let values_i32: Arc<RwLock<Vec<i32>>> = Arc::new(RwLock::new(vec![]));
    let (tx, rx): (mpsc::Sender<ValueSet>, mpsc::Receiver<ValueSet>) = mpsc::channel();
    let values_clone = Arc::clone(&values_i32);
    //Spawns value manager
    spawn(move || loop {
        let value = rx.recv().unwrap();
        let mut values = values_clone.write().unwrap();
        values[value.id] = value.value;
    });
    return (tx, values_i32);
}

/// A WebSocket echo server
fn main() {
    let valueChanger = value_manager_setup();

    let server = TcpListener::bind("127.0.0.1:9001").unwrap();

    for stream in server.incoming() {
        spawn(move || {
            let mut websocket = accept(stream.unwrap()).unwrap();
            loop {
                let msg = websocket.read().unwrap();

                // We do not want to send back ping/pong messages.
                if msg.is_binary() || msg.is_text() {
                    print!("Received: {}", msg);
                    websocket.send(msg).unwrap();
                }
            }
        });
    }
}
