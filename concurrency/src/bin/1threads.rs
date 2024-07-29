use std::{thread, time::Duration};

fn main() {
    premature_exiting();
    println!("\nsync_exiting:");
    sync_exiting();
}

fn premature_exiting() {
    thread::spawn( || {
        for i in 0..5 {
            println!("premature | spawned thread. #{i}");
            thread::sleep(Duration::from_millis(1));
        }
    });

    for i in 0..2 {
        println!("premature | main thread. #{i}");
        thread::sleep(Duration::from_millis(1));
    }
}

fn sync_exiting() {
    let mut num = 0;
    // add `move` to closure to use variables outside of it.
    let handle = thread::spawn( move || {
        for i in 0..10 {
            num += num;
            println!("sync | spawned thread. #{i}");
            thread::sleep(Duration::from_millis(1));
        }
        dbg!(num);
    });
    dbg!(&num);
    for i in 0..5 {
        println!("sync | main thread. #{i}");
        thread::sleep(Duration::from_millis(1));
    }
    handle.join().unwrap(); // ensures main and spawned thread will finish together
}