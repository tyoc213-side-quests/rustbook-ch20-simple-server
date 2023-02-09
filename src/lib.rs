use std::{thread, sync::{mpsc, Arc, Mutex}};

pub struct Worker {
    id: usize,
    thread: Option<thread::JoinHandle<()>>
}

impl Worker {
    // pub fn new(id: usize, receiver: mpsc::Receiver<Job>) -> Worker {
    //     let thread = thread::spawn(||{receiver});
    //     Worker {id, thread}
    // }
    fn new(id: usize, receiver: Arc<Mutex<mpsc::Receiver<Job>>>) -> Worker {
        let thread = thread::spawn(move || loop {
            let message = receiver.lock().unwrap().recv();

            match message {
                Ok(job) => {
                    println!("Worker {id} got a job; executing.");
                },
                Err(_) => {
                    println!("worker {id} disconnected; cant execute job");
                    break;
                }
            }
        });
        Worker { id, thread: Some(thread) }
    }
}
pub struct ThreadPool {
    workers: Vec<Worker>,
    sender: Option<mpsc::Sender<Job>>,
}

type Job = Box<dyn FnOnce() + Send + 'static>;

impl Drop for ThreadPool {
    fn drop(&mut self) {
        drop(self.sender.take());

        for worker in &mut self.workers {
            println!("Shuting down worker {:?}", worker.id);

            if let Some(t) = worker.thread.take() {
                println!("finalizando {:?}", t);
                t.join().unwrap();
            }
        }
    }
}

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
        ThreadPool { workers, sender: Some(sender) }
    }

    pub fn execute<F>(&self, f: F)
    where F: FnOnce() + Send + 'static,{
        let job = Box::new(f);
        self.sender.as_ref().unwrap().send(job).unwrap();
    }

    // TODO: pub fn build(size: usize) -> Result<ThreadPool, PoolCreationError> {...}
}




