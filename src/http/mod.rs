use async_std::prelude::*;
use async_std::net::TcpStream;

#[derive(Debug)]
pub struct RequestConfig {
    pub method: &'static str,
    pub url: &'static str,
}

pub async fn request(addr: String, config: RequestConfig) -> Result<String, Box<dyn std::error::Error>> {
    let mut stream = TcpStream::connect(addr).await?;

    let head_line = format!("{} {} HTTP/1.1\n\n", config.method, config.url);
    stream.write_all(head_line.as_bytes()).await?;
    // stream.flush()?;

    let mut buffer = Vec::new();
    // FIX: 不知道为什么read_to_end要等好久
    stream.read_to_end(&mut buffer).await?;

    let content = String::from_utf8(buffer)?;
    Ok(content)
}
