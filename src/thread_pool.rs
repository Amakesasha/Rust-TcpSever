use std::{
    sync::{mpsc, Arc, Mutex},
    thread,
};

pub struct ThreadPool {
    /// Threads Job.
    workers: Vec<Worker>,
    /// Thread Send-Read.
    sender: mpsc::Sender<JobForWorkers>,
    /// Min number Workers.
    size: usize,
}

/// Number Job, which do Workers.
pub static mut NUM_JOB_FOR_WORKERS: usize = 0;
/// Job, for Workers.
pub type JobForWorkers = Box<dyn FnOnce() + Send + 'static>;

/// Functions to Make New Struct, Add Job and Add Worker.
impl ThreadPool {
    #[inline]
    /// Make a New Thread Pool, and Make Min (size) Worker.
    /// * size = Min number Worker.
    pub fn new(size: usize) -> ThreadPool {
        assert!(size >= 2);

        let (sender, receiver) = mpsc::channel();
        let receiver = Arc::new(Mutex::new(receiver));
        let workers = (0..size)
            .map(|_id| Worker::new(Arc::clone(&receiver)))
            .collect();

        ThreadPool {
            workers,
            sender,
            size,
        }
    }

    #[inline]
    /// Send new Job. Check Number of Worker per Job.
    /// * When Jobs become More or Qqual to the Number of Worker, Workers are Added until they can handle all of them.
    /// * When there are Fewer Jobs than the Number of Worker, Worker are Removed, Leaving one Spare. 
    pub fn execute<F>(&mut self, f: F)
    where
        F: FnOnce() + Send + 'static,
    {
        self.sender.send(Box::new(f)).unwrap();

        let num_jfw = unsafe { NUM_JOB_FOR_WORKERS };

        while num_jfw >= self.workers.len() && self.workers.len() > 0 {
            self.add_worker();
        }

        while num_jfw < self.workers.len() && self.workers.len() > self.size {
            match self.workers.pop() {
                Some(_) => {}
                None => {}
            };
        }
    }

    #[inline]
    /// Add a New Worker to Vector Workers. 
    pub fn add_worker(&mut self) {
        let receiver = Arc::clone(&self.workers[0].receiver);
        let new_worker = Worker::new(receiver);
        self.workers.push(new_worker);
    }
}

/// Impl Trait Drop for ThreadPool
impl Drop for ThreadPool {
    #[inline]
    /// Drop Thread Pool.
    fn drop(&mut self) {
        for worker in &mut self.workers {
            if let Some(thread) = worker.thread.take() {
                thread.join().unwrap_or(());
            }
        }
    }
}

/// Worker (Thread do Job).
struct Worker {
    /// Thread Job.
    thread: Option<thread::JoinHandle<()>>,
    /// Thread Send-Read.
    receiver: Arc<Mutex<mpsc::Receiver<JobForWorkers>>>,
}

/// Function Make a new Worker.
impl Worker {
    #[inline]
    /// Make a new Worker.
    /// * receiver = Thread Send-Read.
    fn new(receiver: Arc<Mutex<mpsc::Receiver<JobForWorkers>>>) -> Worker {
        let receiver_clone = Arc::clone(&receiver);

        let thread = thread::spawn(move || loop {
            let message = match receiver_clone.lock() {
                Ok(message) => message.recv(),
                Err(_) => continue,
            };

            match message {
                Ok(job) => {
                    unsafe {
                        NUM_JOB_FOR_WORKERS += 1;
                    }
                    job();
                    unsafe {
                        if NUM_JOB_FOR_WORKERS > 0 {
                            NUM_JOB_FOR_WORKERS -= 1;
                        }
                    }
                }
                Err(_) => break,
            }
        });

        Worker {
            thread: Some(thread),
            receiver,
        }
    }
}