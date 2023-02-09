use std::thread;

pub struct ThreadPool {
    threads: Vec<thread::JoinHandle<()>>
}

impl ThreadPool {
    /// Creates a new [`ThreadPool`].
    ///
    /// # Panics
    ///
    /// Panics if size is 0.
    pub fn new(size: usize) -> ThreadPool {
        assert!(size > 0);  // a pool of 0 should not be recoverable with a Result

        let mut threads = Vec::with_capacity(size);

        for _ in 0..size {
            // ...
        }
        ThreadPool { threads }
    }

    pub fn execute<F>(&self, f: F)
    where F: FnOnce() + Send + 'static,{

    }

    // TODO: pub fn build(size: usize) -> Result<ThreadPool, PoolCreationError> {...}
}