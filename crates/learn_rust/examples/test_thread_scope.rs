use std::thread::scope;

fn main() {
    let mut count = 0;

    let sub_thread = || {
        for i in 1..10 {
            count += i;
        }
        println!("sub thread");
    };

    scope(|s| {
        s.spawn(sub_thread);
        println!("Main thread");
    });

    println!("count: {count}");
}
