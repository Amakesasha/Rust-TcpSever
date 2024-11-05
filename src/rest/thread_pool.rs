use crate::*;

/// ThreadPool.
pub struct ThreadPool {
    /// Executing threads.
    pub workers: Vec<Worker>,
    /// Channel for transferring work to the executing thread.
    pub sender: mpsc::Sender<WorkThreads>,
}

/// Work for execution threads.
pub type WorkThreads = Box<dyn FnOnce() + Send + 'static>;

/// Functions for creating [ThreadPool] and working with it.
impl ThreadPool {
    #[inline]
    /// Creating a new ThreadPool
    /// * num_thr = Quantity execution threads.
    /// # Examples
    /// ```
    /// let thread_pool = ThreadPool::new(4);
    /// ```
    pub fn new(num_thr: usize) -> ThreadPool {
        assert!(num_thr > 0);

        let (sender, receiver) = mpsc::channel();
        let receiver = Arc::new(Mutex::new(receiver));
        let workers = (0..num_thr)
            .map(|_id| Worker::new(Arc::clone(&receiver)))
            .collect();

        ThreadPool {
            workers,
            sender,
        }
    }

    #[inline]
    /// Sending work to executing threads.
    /// # Examples
    /// ```
    /// let thread_pool = ThreadPool::new(4);
    /// thread_pool.add(|| println!("This Work!") );
    /// ```
    pub fn add<F: FnOnce() + Send + 'static>(&self, f: F) {
        let job = Box::new(f);

        if let Err(e) = self.sender.send(job) {
            eprintln!("THREAD_POOL | ERROR | SENDING | {e:?}");
        }
    }

    #[inline]
    /// Run multiple functions.
    /// * vec_fn = Vector Functions.
    /// # Examples
    /// ```
    /// ThreadPool::launch(
    ///     || println!("This Work 1!"),
    ///     || println!("This Work 2!"),
    ///     || println!("This Work 3!"),
    /// );
    /// ```
    pub fn launch<F: FnOnce() + Send + 'static>(vec_fn: Vec<F>) {
        let thread_pool = ThreadPool::new(vec_fn.len());

        for function in vec_fn {
            thread_pool.add(function);
        }
    }
}

impl Drop for ThreadPool {
    #[inline]
    fn drop(&mut self) {
        for worker in &mut self.workers {
            if let Some(thread) = worker.0.take() {
                if let Err(e) = thread.join() {
                    eprintln!("THREAD_POOL | ERROR | SHOT_DOWN | {e:?}");
                }
            }
        }
    }
}

/// Execution threads.
pub struct Worker ( Option<thread::JoinHandle<()>> );

/// Functions for creating new execution threads.
impl Worker {
    #[inline]
    /// Functions for creating new execution threads.
    /// * receiver = Work acceptance thread.
    pub fn new(receiver: Arc<Mutex<mpsc::Receiver<WorkThreads>>>) -> Worker {
        let thread = thread::spawn(move || loop {
            let message = match receiver.lock() {
                Ok(data) => data.recv(),
                Err(_) => break,
            };

            match message {
                Ok(job) => job(),
                Err(_) => break,
            }
        });

        Worker (Some(thread))
    }
}
