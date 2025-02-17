use conf::Conf;
use serde::Deserialize;
use std::net::SocketAddr;
use std::path::{Path, PathBuf};
use std::str::FromStr;
use utils::ip;
use utils::ip::SocketIp;

pub const DEFAULT_PATH: &str = "./server_conf.toml";
const DEFAULT_MUSIC: &str = "./music.wav";
#[derive(serde::Serialize, Deserialize)]
pub struct ServerConf {
    ip: ip::SocketIp,
    path: PathBuf,
    threads: usize,
}

impl ServerConf {
    pub fn ip(&self) -> &SocketAddr {
        &self.ip
    }
    pub fn path(&self) -> &PathBuf {
        &self.path
    }
    pub fn threads(&self) -> usize {
        self.threads
    }
}
impl Conf for ServerConf {}
pub fn read_conf<P: AsRef<Path>>(path: P) -> ServerConf {
    ServerConf::read_from_path(path).unwrap_or_else(|_e| {
        let default = ServerConf::default();
        default.set_config_to_path(DEFAULT_PATH).unwrap();
        default
    })
}
pub fn read_conf_with_message<P: AsRef<Path>>(path: P) -> ServerConf {
    println!("读取配置中...");
    let conf = read_conf(path);
    println!("配置读取成功");
    conf
}

impl Default for ServerConf {
    fn default() -> Self {
        ServerConf {
            ip: SocketIp::default(),
            path: PathBuf::from_str(DEFAULT_MUSIC).unwrap(),
            threads: 3,
        }
    }
}
