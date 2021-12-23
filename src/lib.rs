use std::thread;

pub struct ThreadPool {
    threads: Vec<thread::JoinHandle<()>>,
}

impl ThreadPool {
    pub fn new(size: usize) -> ThreadPool {
        assert!(size > 0);

        let mut threads = Vec::with_capacity(size);

        for _ in 0..size {
            // create some threads and store them in the vector
            threads.push(thread::spawn(||{}))
        }

        ThreadPool { threads }
    }
    pub fn execute<F, T>(&self, f: F) -> JoinHandle<T>
    where
        F: FnOnce() + Send + 'static,
        T: Send + 'static,
    {
    }
}
