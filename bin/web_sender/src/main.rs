use std::{
    io::prelude::*,
    io::{Read, Write},
    net::TcpStream,
};

use choose_file::choose_file;
use entry::non_empty_entrys;

fn main() {
    //配置
    let conf_result = config::Conf::read_from_path("./config.toml");
    let conf = match conf_result {
        Ok(o) => o,
        Err(e) => {
            eprintln!("读取配置失败: {e}");
            eprintln!("尝试写入新配置...");
            let de = config::Conf::default();
            de.set_config_to_path("./config.toml")
                .expect("尝试写入失败");
            eprintln!("写入成功 位置: ./config.toml");
            de
        }
    };

    //选择文件
    let mut entrys;
    while let None = {
        entrys = non_empty_entrys(&conf.search_dir.0, &conf.suffix);
        &entrys
    } {
        eprintln!(
            "未找到后缀为:{}的文件, 请放好文件后按回车键",
            &conf.suffix.0
        );
        web_sender::pause();
    }
    let entrys = entrys.unwrap();
    let entrys = show_time::sort_by_time_new_to_old(&entrys);
    show_time::show_name_and_time(&entrys);

    let mut entry;
    while let Err(e) = {
        entry = choose_file(&entrys);
        &entry
    } {
        eprintln!("文件选择错误: {e}")
    }
    let entry = entry.unwrap();

    let mut f = std::fs::OpenOptions::new()
        .read(true)
        .open(entry.path())
        .unwrap();

    //网络服务
    let mut buf = String::new();
    f.read_to_string(&mut buf).unwrap();
    let lis = std::net::TcpListener::bind((conf.ip.ip, conf.ip.proxy.0));
    let lis = lis.expect(format!("未能成功监听:{} {}", conf.ip.ip, conf.ip.proxy.0).as_str());
    eprintln!("文件:{}", entry.file_name().to_str().unwrap());
    eprintln!("http://{}:{}", conf.ip.ip, conf.ip.proxy.0);
    let tp = threadpool::ThreadPool::new(conf.thread_number);
    // lis.incoming().flatten().for_each(move |mut t| {
    //     tp.execute(move || {
    //         t.write_all(buf.as_str().as_bytes()).unwrap();
    //         t.flush().unwrap();
    //     })
    //     .unwrap();
    // });

    // for t in lis.incoming() {
    //     let mut t = t.unwrap();
    //     tp.execute(move || {
    //         t.write_all(&buf.as_str().as_bytes()).unwrap();
    //         t.flush().unwrap();
    //     });
    // }
    while let Ok(t) = lis.accept() {
        let buf = buf.clone();
        tp.execute(move || {
            eprintln!("任务来自:{}", t.1.ip().to_string());
            handle_connection(t.0, buf);
        })
        .unwrap();
    }
}
fn handle_connection(mut ts: TcpStream, s: String) {
    let _a = std::io::BufReader::new(&mut ts)
        .lines()
        .next()
        .unwrap()
        .unwrap();
    let status_line = "HTTP/1.1 200 OK";
    let contents = s;
    let length = contents.len();

    let response = format!("{status_line}\r\nContent-Length: {length}\r\n\r\n{contents}");

    // let response = s;
    ts.write_all(response.as_bytes()).unwrap();
    ts.flush().unwrap();
}
