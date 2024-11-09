use choose_file::choose_file;
use config::Conf;
use entry::non_empty_entrys;

pub fn get_config() -> config::Conf {
    let conf_result = config::Conf::read_from_path("./config.toml");
    match conf_result {
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
    }
}

pub fn choose_send_file(conf: &Conf) -> walkdir::DirEntry {
    let mut entrys;
    while let None = {
        entrys = non_empty_entrys(&conf.search_dir.0, &conf.suffix);
        &entrys
    } {
        eprintln!(
            "未找到后缀为:{}的文件, 请放好文件后按回车键",
            &conf.suffix.0
        );
        utils::pause();
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
    entry.unwrap()
}

pub fn get_string_from_file(entry: &walkdir::DirEntry) -> std::sync::Arc<String> {
    use std::io::prelude::*;
    let mut f = std::fs::OpenOptions::new()
        .read(true)
        .open(entry.path())
        .unwrap();

    //网络服务
    let mut buf = String::new();
    f.read_to_string(&mut buf).unwrap();
    std::sync::Arc::new(buf)
}

pub fn response_connection(
    lis: std::net::TcpListener,
    tp: std::sync::Arc<std::sync::RwLock<threadpool::ThreadPool>>,
    buf: std::sync::Arc<String>,
) -> anyhow::Result<()> {
    lis.incoming()
        .flatten()
        .map(move |mut t| -> anyhow::Result<()> {
            use std::io::prelude::*;
            let _a = std::io::BufReader::new(&mut t)
                .lines()
                .next()
                .unwrap()
                .unwrap();
            if _a == "关闭线程池" && t.peer_addr().unwrap().ip().to_string() == "127.0.0.1" {
                eprintln!("正在关闭分发线程");
                tp.write()
                    .unwrap()
                    .execute_then_close(|| eprintln!("正在关闭线程池"))
                    .unwrap();
                eprintln!("已关闭线程池");
                eprintln!("已关闭分发线程");
                std::process::exit(0);
                // return Err(anyhow::Error::msg("分发线程已关闭"));
            } else {
                let buf = buf.clone();
                tp.read().unwrap().execute(move || {
                    eprintln!(
                        "任务来自:{}:{}",
                        t.peer_addr().unwrap().ip().to_string(),
                        t.peer_addr().unwrap().port().to_string()
                    );
                    if let Err(e) = handle_connection(t, buf) {
                        eprintln!("处理链接时出错: {e}");
                    };
                })?;
            }
            anyhow::Ok(())
        })
        .filter(|result| result.is_err())
        .for_each(|e| eprintln!("任务执行错误: {}", e.err().unwrap()));

    anyhow::Ok(())
}

fn handle_connection(mut ts: std::net::TcpStream, s: std::sync::Arc<String>) -> anyhow::Result<()> {
    use std::io::prelude::*;
    // let _a = std::io::BufReader::new(&mut ts)
    //     .lines()
    //     .next()
    //     .unwrap()
    //     .unwrap();
    // dbg!(&_a);

    let status_line = "HTTP/1.1 200 OK";
    let contents = s;
    // let contents = "这是来自服务器的回复\na";
    let length = contents.len();
    let response = format!("{status_line}\r\nContent-Length: {length}\r\n\r\n{contents}");

    // let response = s;
    ts.write_all(response.as_bytes())?;
    ts.flush()?;

    anyhow::Ok(())
}

pub fn close_the_sender_thread(conf: &config::Conf) {
    use std::io::prelude::*;
    use std::net;

    let mut ts = net::TcpStream::connect(format!("{}:{}", conf.ip.ip, conf.ip.proxy.0)).unwrap();
    ts.write_all("关闭线程池\n".as_bytes()).unwrap();
    ts.flush().unwrap();
}
