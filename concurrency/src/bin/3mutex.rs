use std::sync::Arc;
use std::sync::Mutex;
use std::thread;
use std::time::Duration;

/*
Mutex (Mutual Exclusion) - is a data structure that can lock/unlock data from accessing
by other threads, and giving access only to one thread by `locking` data to thread.
*/
fn main() {
    let m = Mutex::new(5);
    {
        // use .lock() to access data:
        let num_ref = m.lock().expect("The lock is acquired by other thread");
        // we tread MutexGuard<i32> as &mut i32, bc it's a smart pointer
        let mut num = *num_ref;
        num += 1;
        println!("num: {num}");
    } // will release a lock when num_ref is dropped.

    multithread_mutex();
}

/* This won't work, because mutex can have only one owner, and value is moved to the 
first thread, instead of getting shared amoung all threads.

fn multithread_mutex_single_owner() {
    let mut handles = vec![];
    let text_mutex = Mutex::new(String::new());

    for i in 0..10 {
        let handle = thread::spawn(move || {
            let mut text = text_mutex.lock().unwrap();
            text.push_str(i.to_string().as_str());
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }
    println!("Result: {}", *text_mutex.lock().unwrap())
} */

// Rc<T> - allows for multiple ownership, but is not thread safe, so
// use Arc<T> (Atomically Reference Counted) instead.
fn multithread_mutex() {
    println!("Multithreaded Mutex:");
    let mut handles = vec![];
    let text_mutex = Arc::new(Mutex::new(String::new()));

    for i in 0..10 {
        let text_mutex = Arc::clone(&text_mutex);
        let handle = thread::spawn(move || {
            let mut text = text_mutex.lock().unwrap();
            text.push_str((i.to_string() + " ").as_str());
            thread::sleep(Duration::from_millis(1));
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }
    println!("Result: `{}`", *text_mutex.lock().unwrap())
} 

// for simple types (int, float), you can use  std::sync::atomic crate 
// to access atomic types - they can be used without `Mutex<T>`.

// Mutex<> is similiar to RefCell<> - it allows to mutate immutable data (interior mutability)