use std::io;
use std::str;

use rust_demo::http::*;

#[tokio::main]
async fn main() {
    let uri = "https://kyfw.12306.cn/otn/resources/js/framework/station_name.js"
        .parse()
        .unwrap();

    let response = fetch_url(uri).await.unwrap();

    let stations = get_content(response).unwrap();
    let stations = stations.split("@").collect::<Vec<&str>>();

    loop {
        println!("please input station name ([enter] to exit):");
        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        let input = input.trim();
        if input == "" {
            break;
        }
        if input.starts_with("search") {
            let input = input.split("\\s+").collect::<Vec<&str>>();
            if input.len() < 2 {
                println!();
                continue;
            }
            let input = input[1];
            let station_list = find_station(input, &stations);
            for station in station_list {
                println!("-> {:?}", station);
            }
        }

        println!();
    }
    println!("see you~");
}
