use std::sync::{
    mpsc::{Receiver, Sender},
    Arc, Mutex,
};

pub struct ThreadPool {
    _list: Vec<Worker>,
    sender: Option<Sender<Job>>,
}
impl ThreadPool {
    pub fn new(size: usize) -> Self {
        let mut v = Vec::with_capacity(size);
        let (send, recv) = std::sync::mpsc::channel();
        let recv = Arc::new(Mutex::new(recv));
        (0..size).for_each(|e| {
            v.push(Worker::new(e, recv.clone()));
        });
        Self {
            _list: v,
            sender: Some(send),
        }
    }
    pub fn execute<F: FnOnce() + Send + 'static>(&self, f: F) -> anyhow::Result<()> {
        if let Err(e) = self.sender.as_ref().unwrap().send(Box::new(f)) {
            return Err(anyhow::Error::msg(format!("执行错误: {}", e.to_string())));
        }
        anyhow::Ok(())
    }
}
impl Drop for ThreadPool {
    fn drop(&mut self) {
        // println!("droping tp");
        self.sender.take();

        while let Some(w) = self._list.pop() {
            drop(w)
        }
    }
}
struct Worker {
    _id: usize,
    handle: Option<std::thread::JoinHandle<()>>,
}
impl Worker {
    fn new(id: usize, recv: Arc<Mutex<Receiver<Job>>>) -> Self {
        let handle = std::thread::spawn(move || loop {
            match recv.lock().unwrap().recv() {
                Ok(o) => o(),
                Err(_e) => {
                    break;
                }
            }
        });
        Self {
            _id: id,
            handle: Some(handle),
        }
    }
}
impl Drop for Worker {
    fn drop(&mut self) {
        // println!("正在销毁线程 id: {}", self.id);
        if let Some(s) = self.handle.take() {
            s.join().unwrap();
        }
    }
}
type Job = Box<dyn FnOnce() + Send + 'static>;
