use std::net::IpAddr;

#[derive(serde::Serialize, serde::Deserialize)]
pub struct Conf {
    config: Config,
}
impl Conf {
    pub fn default() -> Self {
        Conf {
            config: Config {
                ip: "127.0.0.1".parse::<IpAddr>().unwrap(),
                proxy: Proxy(3000),
            },
        }
    }
}
#[derive(serde::Serialize, serde::Deserialize)]
struct Config {
    ip: IpAddr,
    proxy: Proxy,
}
#[derive(serde::Serialize)]
struct Proxy(u16);

impl<'de> serde::Deserialize<'de> for Proxy {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        deserializer.deserialize_u16(ProxyVisitor)
    }
}

struct ProxyVisitor;
impl<'de> serde::de::Visitor<'de> for ProxyVisitor {
    type Value = Proxy;
    fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        formatter.write_str("ProxyVisitor expecting出现异常")
    }
    fn visit_u16<E>(self, v: u16) -> Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        Ok(Proxy(v))
    }
}
