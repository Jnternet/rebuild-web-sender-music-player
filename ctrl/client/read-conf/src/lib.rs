use conf::Conf;
use std::net::SocketAddr;
use std::path::Path;
use utils::ip::*;
pub const DEFAULT_PATH: &str = "./client_conf.toml";
#[derive(serde::Serialize, serde::Deserialize, Debug, Default)]
pub struct ClientConf {
    ip: SocketIp,
}

impl ClientConf {
    pub fn ip(&self) -> &SocketAddr {
        &self.ip
    }
}
impl Conf for ClientConf {}

pub fn read_conf<P: AsRef<Path>>(path: P) -> ClientConf {
    ClientConf::read_from_path(path).unwrap_or_else(|_e| {
        let default = ClientConf::default();
        default.set_config_to_path(DEFAULT_PATH).unwrap();
        default
    })
}
pub fn read_conf_with_message<P: AsRef<Path>>(path: P) -> ClientConf {
    println!("读取配置中...");
    let conf = read_conf(path);
    println!("配置读取成功");
    conf
}
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
