use std::{
    io::{Read, Write},
    net::IpAddr,
    path::Path,
};

#[derive(serde::Serialize, serde::Deserialize, Debug)]
pub struct Conf {
    ip: Ip,
    suffix: Suffix,
}
impl Conf {
    pub fn default() -> Self {
        Conf {
            ip: Ip {
                ip: "127.0.0.1".parse::<IpAddr>().unwrap(),
                proxy: Proxy(3000),
            },
            suffix: Suffix(".yaml".to_string()),
        }
    }
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
            .create(true)
            .open(path)?;

        f.write_all(s.as_bytes())?;
        f.flush()?;

        anyhow::Ok(())
    }
}
#[derive(serde::Serialize, serde::Deserialize, Debug)]
struct Ip {
    ip: IpAddr,
    proxy: Proxy,
}
#[derive(serde::Serialize, serde::Deserialize, Debug)]
struct Proxy(u16);

#[derive(serde::Serialize, serde::Deserialize, Debug)]
struct Suffix(String);
