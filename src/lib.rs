use std::thread;

pub struct Worker {
    id: usize,
    handle: thread::JoinHandle<()>
}

impl Worker {
    pub fn new(id: usize) -> Worker {
        let handle = thread::spawn(||{});        
        Worker {id, handle}
    }
}
pub struct ThreadPool {
    threads: Vec<Worker>
}

impl ThreadPool {
    /// Creates a new [`ThreadPool`].
    ///
    /// # Panics
    ///
    /// Panics if size is 0.
    pub fn new(size: usize) -> ThreadPool {
        assert!(size > 0);  // a pool of 0 should not be recoverable with a Result

        let mut threads: Vec<Worker> = Vec::with_capacity(size);

        for idx in 0..size {
            let w = Worker::new(idx);
            threads.push(w);
        }
        ThreadPool { threads }
    }

    pub fn execute<F>(&self, f: F)
    where F: FnOnce() + Send + 'static,{

    }

    // TODO: pub fn build(size: usize) -> Result<ThreadPool, PoolCreationError> {...}
}