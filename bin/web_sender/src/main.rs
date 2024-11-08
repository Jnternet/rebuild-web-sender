fn main() {
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

    let entrys = entry::get_direntry_with_suffix(conf.search_dir.0, conf.suffix);
    show_time::show_name_and_time(&entrys);
}
