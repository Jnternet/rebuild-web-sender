#[allow(dead_code)]
pub fn a() {
    use std::fs;
    use std::time;

    let mdata = fs::metadata("test.txt").unwrap();
    let modify = mdata.modified().unwrap();
    // let modify = mdata.modified().unwrap();
    let e = modify.duration_since(time::UNIX_EPOCH).unwrap();
    let du = dbg!(chrono::Duration::from_std(e).unwrap());
    let t = chrono::DateTime::from_timestamp_nanos(du.num_nanoseconds().unwrap());
    dbg!(t);
    let 北京时 = t.with_timezone(&chrono::FixedOffset::east_opt(8 * 3600).unwrap());
    let v = 北京时.format("%Y年%m月%d日 %H:%M:%S(北京时)");
    println!("v: {v}")
}

#[allow(dead_code)]
pub fn show_name_and_time() {
    show_time::show_name_and_time(&entry::get_direntry_with_suffix("./", ".yaml"));
}

pub mod ttoml {
    use std::{
        io::{Read, Write},
        net::IpAddr,
    };

    #[derive(serde::Deserialize, serde::Serialize, Debug)]
    struct Conf {
        config: Config,
    }
    #[derive(serde::Deserialize, serde::Serialize, Debug)]
    struct Config {
        ip: IpAddr,
        proxy: u16,
    }
    #[allow(dead_code)]
    pub fn dump_conf() {
        let c = Config {
            ip: "127.0.0.1".parse::<IpAddr>().unwrap(),
            proxy: 25565,
        };
        let conf = Conf { config: c };
        let s = toml::to_string_pretty(&conf).unwrap();
        dbg!(&s);
        let mut f = std::fs::OpenOptions::new()
            .write(true)
            .create(true)
            .open("./test_conf.toml")
            .unwrap();
        dbg!(f.write_all(s.as_bytes()).unwrap());
        dbg!(f.flush().unwrap());

        // todo!()
    }
    pub fn dese_conf() {
        let mut f = std::fs::OpenOptions::new()
            .read(true)
            .open("./test_conf.toml")
            .unwrap();
        let mut buf = String::new();
        f.read_to_string(&mut buf).unwrap();
        let t: Conf = toml::from_str(&buf).unwrap();
        dbg!(&t);
    }
}
