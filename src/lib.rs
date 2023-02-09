use std::{thread, sync::{mpsc, Arc, Mutex}};

pub struct Worker {
    id: usize,
    thread: thread::JoinHandle<()>
}

impl Worker {
    // pub fn new(id: usize, receiver: mpsc::Receiver<Job>) -> Worker {
    //     let thread = thread::spawn(||{receiver});
    //     Worker {id, thread}
    // }
    fn new(id: usize, receiver: Arc<Mutex<mpsc::Receiver<Job>>>) -> Worker {
        let thread = thread::spawn(|| {
            receiver;
        });
        Worker { id, thread }
    }
}
pub struct ThreadPool {
    workers: Vec<Worker>,
    sender: mpsc::Sender<Job>,
}

struct Job;

impl ThreadPool {
    /// Creates a new [`ThreadPool`].
    ///
    /// # Panics
    ///
    /// Panics if size is 0.
    pub fn new(size: usize) -> ThreadPool {
        assert!(size > 0);  // a pool of 0 should not be recoverable with a Result

        let (sender, receiver) = mpsc::channel();

        let receiver = Arc::new(Mutex::new(receiver));

        let mut workers = Vec::with_capacity(size);

        for idx in 0..size {
            workers.push(Worker::new(idx, Arc::clone(&receiver)));
        }
        ThreadPool { workers, sender }
    }

    pub fn execute<F>(&self, f: F)
    where F: FnOnce() + Send + 'static,{

    }

    // TODO: pub fn build(size: usize) -> Result<ThreadPool, PoolCreationError> {...}
}




