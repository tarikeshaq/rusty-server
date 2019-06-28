use std::thread;
use std::sync::mpsc;
use std::sync::Mutex;
use std::sync::Arc;
pub struct ThreadPool {
    workers: Vec<Worker>,
    sender: mpsc::Sender<Message>
}


trait FnBox {
    fn call_box(self: Box<Self>);
}

enum Message {
    NewJob(Job),
    Terminate
}

impl<F: FnOnce()> FnBox for F {
    fn call_box(self: Box<F>) {
        (*self)()
    }
}

type Job = Box<dyn FnBox + Send + 'static>;

impl ThreadPool {
    /// Create a new thread pool
    /// 
    /// The size is the number of threads in the pool
    /// 
    /// #Panics
    /// 
    /// The 'new' function will panic if the size is 0
    pub fn new(size: usize) -> ThreadPool {
        assert!(size > 0);

        let mut workers = Vec::with_capacity(size);
        let (sender, receiver) = mpsc::channel();
        let mutex = Arc::new(Mutex::new(receiver));
        for i in 0..size {
            workers.push(Worker::new(i, Arc::clone(&mutex)));
        }
        ThreadPool {
            workers,
            sender
        }
    }

    pub fn execute<F>(&self, f: F)
    where 
    F: FnOnce() + Send + 'static 
    {
        let job = Message::NewJob(Box::new(f));
        self.sender.send(job).unwrap();
    }
}

impl Drop for ThreadPool {
    fn drop(&mut self) {
        for _ in &mut self.workers {
            self.sender.send(Message::Terminate).unwrap();
        }

        for worker in &mut self.workers {
            println!("Shutting down worker {}",worker.id);
            worker.thread.take().unwrap().join().unwrap();
        }
    }
}

struct Worker {
    id: usize,
    thread: Option<thread::JoinHandle<()>>,
}

impl Worker {
    pub fn new(id: usize, reciever: Arc<Mutex<mpsc::Receiver<Message>>>) -> Worker {
        let thread = thread::spawn(move||{
            loop {
                let message = reciever.lock().unwrap().recv().unwrap();
                match message {
                    Message::NewJob(job) => {
                        println!("Worker {} got a new job",id);
                        job.call_box();
                    },
                    Message::Terminate => {
                        break;
                    }
                }
            }
        });

        Worker {
            id,
            thread: Some(thread),
        }
    }
}