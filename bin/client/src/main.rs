use client_play::WebMusic;
use std::process::exit;

fn main() {
    //初始化日志
    let _guard = utils::log::init_log("log/", "client-log");
    //read conf
    let conf = read_conf::read_conf_with_message(read_conf::DEFAULT_PATH);
    show_conf::show_conf(&conf);

    //get connection
    utils::log::tracing::info!("开始尝试连接服务器:{}", conf.ip());
    let connection = std::net::TcpStream::connect(conf.ip());
    if let Err(e) = connection {
        utils::log::tracing::error!("连接出错，请稍后重试:{e}");
        utils::pause();
        exit(1)
    }
    utils::log::tracing::info!("连接成功");
    let connection = connection.unwrap();

    //获得音乐数据
    utils::log::tracing::info!("正在接收数据...");
    let music = WebMusic::from_web(connection).unwrap();
    utils::log::tracing::info!("接收了{}字节的数据", music.size());
    //获得播放设备
    utils::log::tracing::info!("检查播放设备中...");
    let (_o, oh) = WebMusic::try_default_outstream().unwrap();
    //开始播放
    music.play_once(oh).unwrap();
    utils::log::tracing::info!("开始播放");
    eprint!("回车退出");
    utils::pause();
    utils::log::tracing::info!("退出程序");
}
