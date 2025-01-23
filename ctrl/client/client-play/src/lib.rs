use rodio::OutputStreamHandle;
use rodio::{Decoder, OutputStream, Source};
use std::io::{Cursor, Read};
use std::net::TcpStream;
use std::path::Path;

pub struct WebMusic {
    decoder: Decoder<Cursor<Vec<u8>>>,
}
impl WebMusic {
    pub fn from_web(mut tcp: TcpStream) -> anyhow::Result<Self> {
        let mut buf = Vec::new();
        tcp.read_to_end(&mut buf)?;
        anyhow::Ok(Self {
            decoder: Decoder::new(Cursor::new(buf))?,
        })
    }
    pub fn from_file<P: AsRef<Path>>(path: P) -> anyhow::Result<Self> {
        let mut buf = Vec::new();
        let mut file = std::fs::File::open(path)?;
        file.read_to_end(&mut buf)?;
        anyhow::Ok(Self {
            decoder: Decoder::new(Cursor::new(buf))?,
        })
    }
    pub fn play_once(self, output_stream_handle: OutputStreamHandle) -> anyhow::Result<()> {
        anyhow::Ok(output_stream_handle.play_raw(self.decoder.convert_samples())?)
    }
    pub fn try_default_outstream() -> anyhow::Result<(OutputStream, OutputStreamHandle)> {
        anyhow::Ok(OutputStream::try_default()?)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn from_file() {
        let o = WebMusic::try_default_outstream().unwrap();
        WebMusic::from_file("E:\\Coding\\Code\\Rust\\rebuild-web-server-music-player\\test.wav")
            .unwrap()
            .play_once(o.1)
            .unwrap();
        utils::pause();
    }

    #[test]
    fn from_web() {
        let o = WebMusic::try_default_outstream().unwrap();
        WebMusic::from_web(std::net::TcpStream::connect("127.0.0.1:3000").unwrap())
            .unwrap()
            .play_once(o.1)
            .unwrap();
        utils::pause();
    }
}
