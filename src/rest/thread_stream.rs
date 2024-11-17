use crate::*;

/// ThreadStream.
pub struct ThreadStream {
    /// Executing threads.
    pub workers: Vec<Worker>,
    /// Channel for transferring [TcpStream] to the executing thread.
    pub sender: mpsc::Sender<TcpStream>,
}

/// Function for working with TcpStream.
pub type FuncPoll = for<'a> fn(&'a mut TcpStream) -> Option<()>;

/// Function for creating [ThreadStream].
impl ThreadStream {
    #[inline]
    /// Creating a new ThreadStream
    /// * num_threads = Quantity execution threads.
    pub fn new(num_threads: usize, function: FuncPoll) -> ThreadStream {
        let (sender, receiver) = mpsc::channel();
        let receiver = Arc::new(Mutex::new(receiver));
        let workers = (0..num_threads)
            .map(|_id| Worker::new(Arc::clone(&receiver), function))
            .collect();

        ThreadStream {
            workers,
            sender,
        }
    }
}

impl AddAssign<TcpStream> for ThreadStream {
    #[inline]
    /// Sending work to executing threads.
    /// * stream = Client IP address.
    fn add_assign(&mut self, stream: TcpStream) {
        if let Err(e) = self.sender.send(stream) {
            eprintln!("THREAD_POOL | ERROR | SENDING | {e:?}");
        }
    }
}

impl Drop for ThreadStream {
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
    /// * function = Function for working in a thread.
    pub fn new(receiver: Arc<Mutex<mpsc::Receiver<TcpStream>>>, function: FuncPoll) -> Worker {
        let thread = thread::spawn(move || loop {
            let message = match receiver.lock() {
                Ok(message) => message.recv(),
                Err(_) => break,
            };

            match message {
                Ok(mut stream) => function(&mut stream),
                Err(_) => break,
            };
        });

        Worker (Some(thread))
    }
}
