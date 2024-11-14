fn main() {
    //配置
    let conf = web_sender::get_config();
    //选择文件
    let entry = web_sender::choose_send_file(&conf);
    let buf = web_sender::get_string_from_file(&entry);
    //获得tcplistener
    let lis = std::net::TcpListener::bind((conf.ip.ip, conf.ip.proxy.0));
    let lis = lis.unwrap_or_else(|_| panic!("未能成功监听:{} {}", conf.ip.ip, conf.ip.proxy.0));
    //提示信息
    eprintln!("文件:{}", entry.file_name().to_str().unwrap());
    eprintln!("http://{}:{}", conf.ip.ip, conf.ip.proxy.0);
    clipboard::set_str_to_clipboard(&format!("http://{}:{}", conf.ip.ip, conf.ip.proxy.0)).unwrap();
    eprintln!("已自动复制到剪切板");
    //创建 threadpool
    let tp = threadpool::ThreadPool::new(conf.thread_number);
    let tp = std::sync::Arc::new(std::sync::RwLock::new(tp));
    //响应请求
    let atp = tp.clone();
    let handle =
        std::thread::spawn(move || -> _ { web_sender::response_connection(lis, atp, buf) });

    eprintln!("回车以尝试中止程序");
    let stdin = std::io::stdin();
    let mut buf = String::new();
    stdin.read_line(&mut buf).unwrap();
    // drop(tp);
    // tp.write()
    //     .unwrap()
    //     .execute_then_close(|| eprintln!("正在关闭线程池"))
    //     .unwrap();
    web_sender::close_the_sender_thread(&conf);
    if let Err(e) = handle.join().unwrap() {
        eprintln!("分发线程出错:{e}")
    }
}
