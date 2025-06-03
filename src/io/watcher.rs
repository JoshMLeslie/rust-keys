use std::env;
use std::sync::mpsc::channel;
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::{Duration, Instant};
// ---
use crate::types;

pub fn spawn_watcher() -> std::sync::mpsc::Sender<types::midi::Message> {
    let threshold_micro_sec = env::var("THRESHOLD_MICRO_SEC")
        .unwrap()
        .parse::<u64>()
        .unwrap();

    let (tx, rx) = channel::<types::midi::Message>();
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
                    println!("Note(s):");
                    for (t, msg) in b.iter() {
                        println!("  {:.3}: {:?}", t, msg);
                    }
                    println!("--");
                    b.clear();
                }

                last = Instant::now(); // reset to avoid repeated flush
            }
        }
    });

    return tx;
}
