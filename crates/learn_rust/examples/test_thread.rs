use std::sync::mpsc::channel;
use std::thread::spawn;

fn main() {
    let (tx, rx) = channel();

    let sub = |x: u64| {
        return move || {
            let mut res: i32 = 1;
            let mut prev: i32 = 0;
            if x > 1 {
                for _ in 1..x {
                    let old_res = res;
                    res = res + prev;
                    prev = old_res;
                }
            }
            tx.send(format!("{res}"))
                .expect("Unable to send on channel");
            return res;
        };
    };

    let worker = spawn(sub(5));

    let receiver_worker = spawn(move ||{
        let value = rx.recv().expect("Unable to receive from channel");
        println!("The value in the channel {value}");
    });

    let res = worker.join().unwrap();
    receiver_worker.join().unwrap();

    println!("Received the value in the main thread: {:?}", res);
}
