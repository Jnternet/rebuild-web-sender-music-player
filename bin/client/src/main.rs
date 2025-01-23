use client_play::WebMusic;
use std::process::exit;

fn main() {
    //read conf
    let conf = read_conf::read_conf_with_message(read_conf::DEFAULT_PATH);
    show_conf::show_conf(&conf);

    //get connection
    println!("开始尝试连接服务器");
    let connection = std::net::TcpStream::connect(conf.ip());
    if let Err(e) = connection {
        eprintln!("连接出错，请稍后重试:{e}");
        utils::pause();
        exit(1)
    }
    println!("连接成功");
    let connection = connection.unwrap();

    //获得音乐数据
    println!("正在接收数据...");
    let music = WebMusic::from_web(connection).unwrap();
    //获得播放设备
    println!("接收成功,检查设备中...");
    let (_o, oh) = WebMusic::try_default_outstream().unwrap();
    //开始播放
    music.play_once(oh).unwrap();
    println!("开始播放，回车退出");
    // client_play::WebMusic::play_once(
    //     client_play::WebMusic::from_web(connection).unwrap(),
    //     client_play::WebMusic::try_default_outstream().unwrap().1,
    // )
    // .unwrap();
    utils::pause()
}
