pub fn show_conf(conf: &read_conf::ClientConf) {
    println!("配置显示如下:");
    println!("ip: {}", conf.ip())
}
