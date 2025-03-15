pub mod log {
    pub use tracing;
    pub use tracing_appender;
    pub use tracing_subscriber;

    pub fn init_log(dir: &str, file_name: &str) -> tracing_appender::non_blocking::WorkerGuard {
        use tracing_subscriber::prelude::*;
        use tracing_subscriber::{filter::LevelFilter, fmt};
        // // 消费log门面日志 转为 tracing Event日志
        // LogTracer::builder()
        //     // .with_max_level(log::LevelFilter::Error)
        //     .init()
        //     .expect("[PEAR] LogTracer 初始化失败");

        // 标准控制台输出layer
        let fmt_layer = fmt::layer()
            .with_level(true)
            // 指定标准控制台输出
            .with_writer(std::io::stdout)
            // 日志等级过滤
            .with_filter(LevelFilter::INFO);

        // 文件 appender 指定日志文件输出目录和文件名前缀
        // daily 指定生成文件名日期到年月日
        // 如： test-log.2023-08-30
        let file_appender = tracing_appender::rolling::daily(dir, file_name);
        // 生成非阻塞写入器
        let (non_blocking, guard) = tracing_appender::non_blocking(file_appender);
        // 文件输出层
        let file_layer = fmt::layer()
            // 移除输出内容中的 颜色或其它格式相关转义字符
            .with_ansi(false)
            .with_writer(non_blocking)
            // 日志等级过滤
            .with_filter(LevelFilter::INFO);

        // 生成注册中心 Registry 绑定多个输出层
        let collector = tracing_subscriber::registry()
            .with(file_layer)
            .with(fmt_layer);

        // 订阅者全局注册
        tracing::subscriber::set_global_default(collector).expect("日志输出集注册错误");

        guard
    }
}
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
