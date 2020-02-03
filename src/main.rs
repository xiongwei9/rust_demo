use async_std::task;
use rust_demo::http::*;

fn main() {
    println!("hello rust!");

    // test_http();
    // test_http();
    // test_http();
    task::block_on(test_http());
    test_http_sync();
}

async fn test_http() {
    let config = RequestConfig {
        method: "GET",
        url: "/",
    };
    match request(String::from("127.0.0.1:8080"), config).await {
        Ok(content) => println!("--------------- request Ok ---------------\n{}", content),
        Err(error) => println!("--------------- request Err ---------------\n{:?}", error),
    }
}

fn test_http_sync() {
    let config = RequestConfig {
        method: "GET",
        url: "/",
    };
    match sync::request(String::from("127.0.0.1:8080"), config) {
        Ok(content) => println!("--------------- request Ok ---------------\n{}", content),
        Err(error) => println!("--------------- request Err ---------------\n{:?}", error),
    }
}
