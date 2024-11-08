use std::io::Write;

#[test]
fn create_config() {
    let d = config::Conf::default();
    let s = toml::to_string_pretty(&d);
    assert!(s.is_ok());
    let s = s.unwrap();
    let mut f = std::fs::OpenOptions::new()
        .write(true)
        .create(true)
        .open("./conf_test.toml")
        .unwrap();

    let write_result = f.write_all(s.as_bytes());
    assert!(write_result.is_ok());
    let flush_result = f.flush();
    assert!(flush_result.is_ok());
}

#[test]
fn read_config() {
    // let mut f = std::fs::OpenOptions::new()
    //     .read(true)
    //     .open("./conf_test.toml")
    //     .unwrap();
    // let mut buf = String::new();
    // f.read_to_string(&mut buf).unwrap();
    // let conf: config::Conf = toml::from_str(&buf).unwrap();
    // dbg!(conf);
    let conf = config::Conf::read_from_path("./conf_test.toml");
    assert!(conf.is_ok());
}

#[test]
fn set_config_to_path() {
    let c = config::Conf::default();
    let r = c.set_config_to_path("./conf_test.toml");
    assert!(r.is_ok());
}
