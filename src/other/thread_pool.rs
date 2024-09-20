use crate::*;

/// System Thread Pool.
pub struct ThreadPool {
    /// Threads Workers.
    pub workers: Vec<Worker>,
    /// Job for Workers.
    pub sender: Option<mpsc::Sender<JobForWorkers>>,
    /// Min number Workers.
    pub num_thr: usize,
}

/// Job, for Workers.
pub type JobForWorkers = Box<dyn FnOnce() + Send + 'static>;

/// Functions to Make New Struct and Add Worker.
impl ThreadPool {
    #[inline]
    /// Make a New Thread Pool, and Make Min (num_thr) Worker.
    /// * num_thr = Min number Worker.
    pub fn new(num_thr: usize) -> ThreadPool {
        assert!(num_thr > 0);

        let (sender, receiver) = mpsc::channel();
        let receiver = Arc::new(Mutex::new(receiver));
        let workers = (0..num_thr)
            .map(|_id| Worker::new(Arc::clone(&receiver)))
            .collect();

        ThreadPool {
            workers,
            sender: Some(sender),
            num_thr,
        }
    }

    #[inline]
    /// Send new Job. 
    pub fn add_job<F>(&self, f: F)
    where
        F: FnOnce() + Send + 'static,
    {
        let job = Box::new(f);

        match self.sender.as_ref() {
            Some(data) => data.send(job).unwrap_or(()),
            None => return,
        }
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
pub struct Worker {
    /// Thread Job.
    pub thread: Option<thread::JoinHandle<()>>,
}

/// Function Make a new Worker.
impl Worker {
    #[inline]
    /// Make a new Worker.
    /// * receiver = Thread Send-Read.
    pub fn new(receiver: Arc<Mutex<mpsc::Receiver<JobForWorkers>>>) -> Worker {
        let thread = thread::spawn(move || loop {
            let message = match receiver.lock() {
                Ok(data) => data.recv(),
                _ => break,
            };

            match message {
                Ok(job) => job(),
                Err(_) => break,
            }
        });

        Worker {
            thread: Some(thread),
        }
    }
}
