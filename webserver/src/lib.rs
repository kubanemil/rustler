use std::{
    sync::{
        mpsc::{self, Receiver, Sender},
        Arc, Mutex,
    },
    thread::{self, JoinHandle},
};

pub struct ThreadPool {
    sender: Option<Sender<Job>>,
    workers: Vec<Worker>,
}

type Job = Box<dyn FnOnce() + Send + 'static>;

impl ThreadPool {
    /// Create a new ThreadPool.
    ///
    /// The size is the number of threads in the pool.
    ///
    /// # Panics
    ///
    /// The `new` function will panic if the size is zero.
    pub fn new(thread_num: usize) -> ThreadPool {
        assert!(thread_num > 0);

        let (sender, receiver) = mpsc::channel::<Job>();
        let receiver_mutex = Arc::new(Mutex::new(receiver));

        let mut workers = Vec::with_capacity(thread_num);

        for id in 0..thread_num {
            let worker = Worker::new(id, receiver_mutex.clone());
            println!("Worker #{} is created!", &worker.id);
            workers.push(worker);
        }

        ThreadPool {
            sender: Some(sender),
            workers,
        }
    }

    pub fn execute<F>(&self, f: F)
    where
        F: FnOnce() + Send + 'static,
    {
        let job = Box::new(f);
        self.sender.as_ref().unwrap().send(job).unwrap();
    }
}

impl Drop for ThreadPool {
    fn drop(&mut self) {
        drop(self.sender.take());

        for worker in &mut self.workers {
            println!("Shutting down worker #{}", &worker.id);

            if let Some(thread) = worker.thread.take() {
                thread.join().unwrap();
            }
        }
    }
}

struct Worker {
    id: usize,
    thread: Option<JoinHandle<()>>,
}

impl Worker {
    fn new(id: usize, receiver_mutex: Arc<Mutex<Receiver<Job>>>) -> Worker {
        let thread = thread::spawn(move || loop {
            let job = receiver_mutex.lock().unwrap().recv();

            match job {
                Err(_) => {
                    println!("Worker #{id} disconnected; exiting...");
                    break;
                }
                Ok(job) => {
                    println!("Worker #{id} got a job; executing");
                    job();
                }
            }
        });

        Worker {
            id,
            thread: Some(thread),
        }
    }
}
