#![allow(unused)]

#[test]
fn make_thread_pool() {
    let tp = threadpool::ThreadPool::new(3);
    dbg!(tp.execute(Box::new(|| println!("from subthread"))));
    dbg!(tp.execute(Box::new(|| println!("from subthread"))));
    dbg!(tp.execute(Box::new(|| println!("from subthread"))));
    dbg!(tp.execute(Box::new(|| println!("from subthread"))));
    dbg!(tp.execute(Box::new(|| println!("from subthread"))));
    dbg!(tp.execute(Box::new(|| println!("from subthread"))));
    dbg!(tp.execute(Box::new(|| println!("from subthread"))));
    dbg!(tp.execute(Box::new(|| println!("from subthread"))));
}
