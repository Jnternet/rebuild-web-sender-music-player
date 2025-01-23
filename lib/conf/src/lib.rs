use std::{
    io::{Read, Write},
    // net::IpAddr,
    path::Path,
};

pub trait Conf: serde::Serialize + serde::de::DeserializeOwned {
    fn read_from_path<P: AsRef<Path>>(path: P) -> anyhow::Result<Self> {
        let path = path.as_ref();
        let mut f = std::fs::OpenOptions::new().read(true).open(path)?;
        let mut buf = String::new();
        f.read_to_string(&mut buf)?;

        anyhow::Ok(toml::from_str(&buf)?)
    }
    fn set_config_to_path<P: AsRef<Path>>(&self, path: P) -> anyhow::Result<()> {
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

// #[derive(serde::Serialize, serde::Deserialize, Debug)]
// pub struct ClientConf {
//     pub ip: Ip,
// }
//
// impl Conf for ClientConf {}
// impl Default for ClientConf {
//     fn default() -> Self {
//         ClientConf {
//             ip: Ip {
//                 ip: "127.0.0.1".parse::<IpAddr>().unwrap(),
//                 proxy: Proxy(3000),
//             },
//         }
//     }
// }
//
// #[derive(serde::Serialize, serde::Deserialize, Debug)]
// pub struct ServerConf {
//     pub ip: Ip,
// }
//
// impl Conf for ServerConf {}
// impl Default for ServerConf {
//     fn default() -> Self {
//         ServerConf {
//             ip: Ip {
//                 ip: "127.0.0.1".parse::<IpAddr>().unwrap(),
//                 proxy: Proxy(3000),
//             },
//         }
//     }
// }
//
// #[derive(serde::Serialize, serde::Deserialize, Debug)]
// pub struct Ip {
//     pub ip: IpAddr,
//     pub proxy: Proxy,
// }
// #[derive(serde::Serialize, serde::Deserialize, Debug)]
// pub struct Proxy(pub u16);
