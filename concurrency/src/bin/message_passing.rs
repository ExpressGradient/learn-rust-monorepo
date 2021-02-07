use std::sync::mpsc;
use std::thread;
use std::time::Duration;

fn main() {
    let ( tx, rx ) = mpsc::channel();
    let tx_clone = mpsc::Sender::clone(&tx);

    thread::spawn(move || {
        let messages: Vec<String> = vec![
            "Hello World!".to_string(),
            "I'm Express".to_string(),
            "Goodbye".to_string()
        ];

        for message in messages {
            tx.send(message).unwrap();
            thread::sleep(Duration::from_secs(2));
        }
    });

    thread::spawn(move || {
        let messages: Vec<String> = vec![
            "Sadness".to_string(),
            "Drowning in Sadness".to_string()
        ];

        for message in messages {
            tx_clone.send(message).unwrap();
            thread::sleep(Duration::from_secs(1));
        }
    });

    /*if let Ok(message) = rx.try_recv() {
        println!("Got {}", message);
    } else {
        println!("No message received");
    }*/

    for received in rx {
        println!("Got: {}", received);
    }
}