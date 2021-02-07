use std::thread;
use std::time::Duration;

fn main() {
    let handle_one = thread::spawn(|| {
        for i in 1..5 {
            println!("Num: {} from HANDLE ONE thread", i);
            thread::sleep(Duration::from_millis(1));
        }
    });

    let vector: Vec<u32> = vec![1, 2, 3, 4, 5];

    let handle_two = thread::spawn(move || {
        for num in vector.iter() {
            println!("Vector num: {} from HANDLE TWO thread", num);
            thread::sleep(Duration::from_millis(1));
        }
    });

    handle_two.join().unwrap();

    for i in 1..5 {
        println!("Num: {} from MAIN thread", i);
        thread::sleep(Duration::from_millis(1));
    }

    handle_one.join().unwrap();
}