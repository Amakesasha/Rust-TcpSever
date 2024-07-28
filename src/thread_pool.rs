use std::{
    sync::{mpsc, Arc, Mutex},
    thread,
};

/// System Thread Pool.
pub struct ThreadPool {
    /// Threads Job.
    pub workers: Vec<Worker>,
    /// Thread Send-Read.
    pub sender: Option<mpsc::Sender<JobForWorkers>>,
    pub receiver: Arc<Mutex<mpsc::Receiver<JobForWorkers>>>,
    /// Min number Workers.
    pub num_thr: usize,
}

/// Number Job, which do Workers.
pub static mut NUM_JOB_FOR_WORKERS: usize = 0;
/// Job, for Workers.
pub type JobForWorkers = Box<dyn FnOnce() + Send + 'static>;

/// Functions to Make New Struct, Add Job and Add Worker.
impl ThreadPool {
    #[inline]
    /// Make a New Thread Pool, and Make Min (num_thr) Worker.
    /// * num_thr = Min number Worker.
    /// * used_static_add = Used add_static_job(true) or add_const_job(false).
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
            receiver: Arc::clone(&receiver),
            num_thr,
        }
    }

    #[inline]
    /// Send new Job. Check Number of Worker per Job.
    /// * When Jobs become More or Qqual to the Number of Worker, Workers are Added until they can handle all of them.
    /// * When there are Fewer Jobs than the Number of Worker, Worker are Removed, Leaving one Spare.
    pub fn add_static_job<F>(&mut self, f: F)
    where
        F: FnOnce() + Send + 'static,
    {
        unsafe {
            NUM_JOB_FOR_WORKERS += 1;
        }

        if unsafe { NUM_JOB_FOR_WORKERS } >= self.workers.len() && self.workers.len() > 0 {
            self.add_worker();
        }

        self.add_const_job(f);

        if unsafe { NUM_JOB_FOR_WORKERS } + 1 < self.workers.len()
            && self.workers.len() > self.num_thr
        {
            drop(self.workers.pop());
        }
    }

    #[inline]
    /// Send new Job. No Check Number Job and Worker
    pub fn add_const_job<F>(&mut self, f: F)
    where
        F: FnOnce() + Send + 'static,
    {
        let job = Box::new(f);

        self.sender.as_ref().unwrap().send(job).unwrap();
    }

    #[inline]
    /// Add a New Worker to Vector Workers.
    pub fn add_worker(&mut self) {
        let new_worker = Worker::new(Arc::clone(&self.receiver));
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
            let message = receiver.lock().unwrap().recv();

            match message {
                Ok(job) => {
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
        }
    }
}
