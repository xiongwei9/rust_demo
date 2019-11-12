use hyper::Client;
use hyper_tls::HttpsConnector;
use regex::Regex;
use std::str;

#[derive(Debug)]
pub struct Station {
    id: i32,
    name: String,
    code: String,
    pinyin: String,
}

pub fn find_station(name: &str, stations: &Vec<&str>) -> Vec<Station> {
    let mut vec = Vec::new();
    for s in stations {
        if s.contains(name) {
            let arr = s.split("|").collect::<Vec<&str>>();
            if arr.len() < 6 {
                println!("error line: {}", s);
                continue;
            }
            vec.push(Station {
                id: arr[5].parse().unwrap(),
                name: arr[1].to_string(),
                code: arr[2].to_string(),
                pinyin: arr[3].to_string(),
            });
        }
    }
    vec
}

pub fn get_content(response: String) -> Result<String, regex::Error> {
    let reg = Regex::new("'(.*)'")?;
    let mut vec = reg.captures_iter(&response);

    let cap = match vec.next() {
        Some(cap) => cap,
        None => {
            return Ok("".to_string());
        }
    };
    let text = String::from(&cap[1]);

    Ok(text)
}

pub async fn fetch_url(url: hyper::Uri) -> Result<String, Box<dyn std::error::Error>> {
    let https = HttpsConnector::new().unwrap();
    let client = Client::builder().build::<_, hyper::Body>(https);

    let res = client.get(url).await?;

    let mut body = res.into_body();
    let mut v = Vec::new();

    while let Some(next) = body.next().await {
        let chunk = next?;
        v.append(&mut chunk.to_vec());
    }
    let res = String::from(str::from_utf8(&v)?);

    Ok(res)
}
