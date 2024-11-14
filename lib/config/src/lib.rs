use std::{
    io::{Read, Write},
    net::IpAddr,
    path::Path,
};

#[derive(serde::Serialize, serde::Deserialize, Debug)]
pub struct Conf {
    pub ip: Ip,
    pub suffix: Suffix,
    pub search_dir: Dir,
    pub thread_number: usize,
}
impl Conf {
    pub fn read_from_path<P: AsRef<Path>>(path: P) -> anyhow::Result<Self> {
        let path = path.as_ref();
        let mut f = std::fs::OpenOptions::new().read(true).open(path)?;
        let mut buf = String::new();
        f.read_to_string(&mut buf)?;

        anyhow::Ok(toml::from_str(&buf)?)
    }
    pub fn set_config_to_path<P: AsRef<Path>>(&self, path: P) -> anyhow::Result<()> {
        let s = toml::to_string_pretty(self)?;

        let path = path.as_ref();
        let mut f = std::fs::OpenOptions::new()
            .write(true)
            .truncate(true)
            .create(true)
            .open(path)?;

        f.write_all(s.as_bytes())?;
        f.flush()?;

        anyhow::Ok(())
    }
}
impl Default for Conf {
    fn default() -> Self {
        Conf {
            ip: Ip {
                ip: "127.0.0.1".parse::<IpAddr>().unwrap(),
                proxy: Proxy(3000),
            },
            suffix: Suffix(".yaml".to_string()),
            search_dir: Dir("./".to_string()),
            thread_number: 3,
        }
    }
}
#[derive(serde::Serialize, serde::Deserialize, Debug)]
pub struct Ip {
    pub ip: IpAddr,
    pub proxy: Proxy,
}
#[derive(serde::Serialize, serde::Deserialize, Debug)]
pub struct Proxy(pub u16);

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone)]
pub struct Suffix(pub String);
#[derive(serde::Serialize, serde::Deserialize, Debug)]
pub struct Dir(pub String);
