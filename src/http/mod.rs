use async_std::net::TcpStream;
use async_std::prelude::*;

#[derive(Debug)]
pub struct RequestConfig {
    pub method: &'static str,
    pub url: &'static str,
}

pub async fn request(
    addr: String,
    config: RequestConfig,
) -> Result<String, Box<dyn std::error::Error>> {
    let mut stream = TcpStream::connect(addr).await?;

    let head_line = format!("{} {} HTTP/1.1\n\n", config.method, config.url);
    stream.write_all(head_line.as_bytes()).await?;
    // stream.flush()?;

    let mut buffer = Vec::new();
    // FIXME: 不知道为什么read_to_end要等好久
    // 测试了一下，rust tcpStream接收http响
    // 应的时候，到最后会等待一会儿，看还有没
    // 有数据没到的，不知道怎么去掉这个等待
    stream.read_to_end(&mut buffer).await?;

    let content = String::from_utf8(buffer)?;
    Ok(content)
}

pub mod sync {
    use super::RequestConfig;
    use std::io::prelude::*;
    use std::net::TcpStream;

    pub fn request(
        addr: String,
        config: RequestConfig,
    ) -> Result<String, Box<dyn std::error::Error>> {
        let mut stream = TcpStream::connect(addr)?;

        let head_line = format!("{} {} HTTP/1.1\n\n", config.method, config.url);
        stream.write(head_line.as_bytes())?;

        let mut buffer = Vec::new();
        stream.read_to_end(&mut buffer)?;
        let content = String::from_utf8(buffer)?;

        Ok(content)
    }
}
