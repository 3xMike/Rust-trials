use std::sync::{Arc, mpsc, Mutex};
use std::thread::{self, JoinHandle, sleep};
use std::convert::TryInto;

type Job = Box<dyn FnOnce() + Send + 'static>;

enum JobMessage {
    StopWorking,
    DoTask(Job),
}
struct Worker {
    pub thread : Option<JoinHandle<()>>,
    pub id : u32,
}

impl Worker {
    pub fn new(id : u32, receiver : Arc<Mutex<mpsc::Receiver<JobMessage>>>) -> Worker {
        let thread = thread::spawn(move || {
            loop {
                let guard = receiver.lock()
                    .expect(&format!("Other thread paniced! My id: {}", id));
                match guard.recv().expect(&format!("Sender dropped channel! My id: {}", id)) {
                    JobMessage::DoTask(job)=> {
                        drop(guard);
                        println!("Do some job, my id is {}", &id);
                        job();
                    },
                    JobMessage::StopWorking => break,
                };
            }
        });
        Worker { thread : Some(thread), id}
    } 
}

pub struct ThreadPull {
    workers : Vec<Worker>,
    job_channel : Arc<Mutex<mpsc::Receiver<JobMessage>>>,
    sender : mpsc::Sender<JobMessage>,
}
//send message on channel : stop or do closure 

impl ThreadPull {
    pub fn new(number_of_threads : u32) -> ThreadPull {
        let mut workers = Vec::with_capacity(number_of_threads.try_into().unwrap());
        let (sender, receiver) = mpsc::channel::<JobMessage>();
        let job_channel = Arc::new(Mutex::new(receiver));
        for id in 0..number_of_threads {
            workers.push(Worker::new(id, job_channel.clone()));
        }
        ThreadPull { workers, job_channel, sender}
    }

    pub fn execute<T>(&self, job : T) 
    where T : FnOnce() + Send + 'static
    {
        self.sender.send(JobMessage::DoTask(Box::new(job))).expect("Pull couldn't send new job");
    }
}

impl Drop for ThreadPull {
    fn drop(&mut self) {
        for _ in &mut self.workers {
            self.sender.send(JobMessage::StopWorking).expect("Pull couldn't send stop message");
        }
        for worker in &mut self.workers {
            if let Some(thread) = worker.thread.take() {
                thread.join().expect(&format!("Pull couln't join the {} thread", worker.id));
                println!("Joined thread with id {}", worker.id);
            }
// When in Worker layed thread : JoinHandle<()>, we can't move out it, so we had to change it to Option
        }
        drop(&mut self.job_channel);
    }
}

pub fn run(){
    {
        let pull = ThreadPull::new(4);
        for _ in 0..20{
            pull.execute(move || {sleep(std::time::Duration::new(2,0))});
        }
    }
    println!("run worked well");
}