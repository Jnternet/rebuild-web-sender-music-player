use std::io::{Read, Write};
use std::net::TcpStream;
use std::sync::Arc;

fn main() {
    //日志记录开始
    let _guard = utils::log::init_log("log/", "server-log");
    //read conf
    let conf = server::read_conf_with_message(server::DEFAULT_PATH);
    //读取文件
    let mut file = std::fs::OpenOptions::new()
        .read(true)
        .open(conf.path())
        .unwrap();
    // let v = Vec::with_capacity(file.metadata().unwrap().len() as usize);
    let mut v = Vec::new();
    file.read_to_end(&mut v).unwrap();

    //准备发送
    let tcp = std::net::TcpListener::bind(conf.ip()).unwrap();
    let mut tp = mini_threadpool::ThreadPool::new(conf.threads());
    let arcv = Arc::new(v);

    //工作
    println!("开始发送:{}", conf.ip());
    tcp.incoming()
        .filter_map(|r| if let Ok(ts) = r { Some(ts) } else { None })
        .for_each(|s| {
            // println!("请求来自:{}", s.peer_addr().unwrap());
            utils::log::tracing::info!("请求来自:{}", s.peer_addr().unwrap());
            let carcv = arcv.clone();
            tp.execute(|| handle_connection(carcv, s)).unwrap()
        });

    //关闭
    tp.execute_then_close(|| println!("线程池已关闭")).unwrap()
}

fn handle_connection(arcv: Arc<Vec<u8>>, mut s: TcpStream) {
    s.write_all(&arcv).unwrap();
    s.flush().unwrap();
    utils::log::tracing::info!("向{}发送了{}字节", s.peer_addr().unwrap(), arcv.len());
    utils::log::tracing::info!("发送完成，断开和{:?}的连接", s.peer_addr().unwrap());
}
