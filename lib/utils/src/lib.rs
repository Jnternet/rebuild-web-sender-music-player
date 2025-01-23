pub mod ip {
    use std::net::SocketAddr;
    use std::ops::Deref;

    // #[derive(serde::Serialize, serde::Deserialize, Debug)]
    // pub struct Ip {
    //     pub ip: std::net::IpAddr,
    //     pub proxy: Proxy,
    // }

    #[derive(serde::Serialize, serde::Deserialize, Debug)]
    pub struct SocketIp {
        ip: SocketAddr,
    }

    impl SocketIp {
        pub fn ip(&self) -> &SocketAddr {
            &self.ip
        }
    }
    // #[derive(serde::Serialize, serde::Deserialize, Debug)]
    // pub struct Proxy(pub u16);
    //
    // impl Default for Ip {
    //     fn default() -> Self {
    //         Self {
    //             ip: std::net::IpAddr::from_str("127.0.0.1").unwrap(),
    //             proxy: Proxy(3000),
    //         }
    //     }
    // }

    // impl AsRef<SocketAddr> for SocketIp {
    //     fn as_ref(&self) -> &SocketAddr {
    //         &self.ip
    //     }
    // }

    impl Deref for SocketIp {
        type Target = SocketAddr;
        fn deref(&self) -> &Self::Target {
            &self.ip
        }
    }

    impl Default for SocketIp {
        fn default() -> Self {
            Self {
                ip: SocketAddr::new("127.0.0.1".parse().unwrap(), 3000),
            }
        }
    }
}

pub fn pause() {
    let _ = std::process::Command::new("cmd.exe")
        .arg("/c")
        .arg("pause")
        .status();
}
