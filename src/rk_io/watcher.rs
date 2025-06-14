use std::env;
use std::sync::mpsc::{Receiver, Sender, channel};
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::{Duration, Instant};
// ---
use crate::types::midi::Message;

pub fn spawn_watcher() -> (Sender<Message>, Receiver<Vec<Message>>) {
    let debug: bool = env::var("DEBUG").unwrap_or_default().eq("true");
    let threshold_micro_sec = env::var("THRESHOLD_MICRO_SEC")
        .unwrap()
        .parse::<u64>()
        .unwrap();

    let (tx, rx) = channel::<Message>();
    // Output channel for batched MIDI messages (Observable stream)
    let (batch_tx, batch_rx) = channel::<Vec<Message>>();

    let batch = Arc::new(Mutex::new(Vec::new()));
    let batch_clone = Arc::clone(&batch);

    thread::spawn(move || {
        let mut last = Instant::now();

        loop {
            if let Ok(msg) = rx.recv_timeout(Duration::from_micros(threshold_micro_sec)) {
                let mut b = batch_clone.lock().unwrap();
                b.push(msg);
                last = Instant::now();
            }

            if last.elapsed() > Duration::from_micros(threshold_micro_sec) {
                let mut b = batch_clone.lock().unwrap();
                if !b.is_empty() {
                    if debug {
                        println!("Note(s):");
                        for (t, msg) in b.iter() {
                            println!("  {:.3}: {:?}", t, msg);
                        }
                        println!("--");
                    }

                    let batch_to_send = b.clone();
                    let res = batch_tx.send(batch_to_send);
                    if res.is_err() {
                        // Receiver dropped, exit thread
                        break;
                    }

                    b.clear();
                }

                last = Instant::now(); // reset to avoid repeated flush
            }
        }
    });

    return (tx, batch_rx);
}
