use std::io::{Read, Write};

fn main() {
    a();
}

fn a() {
    use std::net;
    let mut ts = net::TcpStream::connect("127.0.0.1:3000").unwrap();
    eprintln!("trying");
    ts.write_all("关闭线程池\n".as_bytes()).unwrap();
    ts.flush().unwrap();
    let mut buf = String::new();
    ts.read_to_string(&mut buf).unwrap();
    dbg!(&buf);
    utils::pause();
}
