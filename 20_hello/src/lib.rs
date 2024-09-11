use std::{
    sync::{mpsc, Arc, Mutex},
    thread,
};

type Job = Box<dyn FnOnce() + Send + 'static>;

pub struct PoolCreationError(String);
pub struct ThreadPool {
    workers: Vec<Worker>,
    sender: Option<mpsc::Sender<Job>>,
}

impl ThreadPool {
    /// Create a new ThreadPool.
    ///
    /// The `size` is the number of threads in the pool.
    /// It is basically a wrapper around the `ThreadPool::build`.
    ///
    /// # Panics
    ///
    /// The `new` function will panic if the `size` is zero.
    pub fn new(size: usize) -> ThreadPool {
        match ThreadPool::build(size) {
            Ok(tp) => tp,
            Err(e) => panic!("PANIC: {}", e.0),
        }
    }

    /// Build a new ThreadPool.
    ///
    /// The `size` is the number of threads in the pool.
    pub fn build(size: usize) -> Result<ThreadPool, PoolCreationError> {
        if size > 0 {
            let (sender, receiver) = mpsc::channel();
            let receiver = Arc::new(Mutex::new(receiver));

            let mut workers = Vec::with_capacity(size);
            for id in 0..size {
                workers.push(Worker::new(id, Arc::clone(&receiver)))
            }

            Ok(ThreadPool {
                workers,
                sender: Some(sender),
            })
        } else {
            Err(PoolCreationError(String::from(
                "`ThreadPool` `size` value cannot be less than 1",
            )))
        }
    }

    /// Execute a job
    ///
    /// Takes a job in the form of a function or a closure and executes it.
    pub fn execute<F>(&self, f: F)
    where
        F: FnOnce() + Send + 'static,
    {
        let job = Box::new(f);
        self.sender
            .as_ref()
            .unwrap()
            .send(job)
            .expect("ERROR: Sending message through channel.");
    }
}

impl Drop for ThreadPool {
    fn drop(&mut self) {
        drop(self.sender.take());

        for worker in &mut self.workers {
            println!("Shutting down worker {}", worker.id);

            if let Some(thread) = worker.thread.take() {
                thread.join().expect("ERROR: Trying to join a thread.");
            }
        }
    }
}
struct Worker {
    id: usize,
    thread: Option<thread::JoinHandle<()>>,
}

impl Worker {
    fn new(id: usize, receiver: Arc<Mutex<mpsc::Receiver<Job>>>) -> Worker {
        let thread = thread::spawn(move || loop {
            let message = receiver
                .lock()
                .expect("ERROR: Trying to get the Mutex lock.")
                .recv();
            if let Ok(job) = message {
                println!("INFO: Worker {id} got a job; executing...");

                job();
            } else {
                println!("INFO: Worker {id} disconnected; shutting down...");

                break;
            }
        });

        Worker {
            id,
            thread: Some(thread),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[should_panic]
    fn threadpool_new_non_zero_threads() {
        ThreadPool::new(0);
    }

    #[test]
    fn threadpool_build_non_zero_threads() {
        if let Ok(_) = ThreadPool::build(0) {
            panic!();
        }
    }

    #[test]
    fn threadpool_execute_function() {
        fn test() -> () {
            println!("Testing...");
            ()
        }

        let tp = ThreadPool::new(1);
        tp.execute(test);
    }

    #[test]
    fn threadpool_execute_closure() {
        let tp = ThreadPool::new(1);
        tp.execute(|| {
            println!("Testing...");
        });
    }
}
