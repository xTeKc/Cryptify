use notify::*;
use http::{Request, Response, StatusCode};


fn req() {
    let request = Request::builder()
    .uri("https://api.coingecko.com/api/v3/simple/price?ids=monero&vs_currencies=usd")
    .header("User-Agent", "awesome/1.0")
    .body(())
    .unwrap();
    println!("{:?}", request);
    }

fn resp() {
    let response = Response::builder()
    .status(StatusCode::MOVED_PERMANENTLY)
    .header("Location", "https://api.coingecko.com/api/v3/simple/price?ids=monero&vs_currencies=usd")
    .body(())
    .unwrap();
    println!("{:?}", response);
}

fn main() {
    req();
    resp();
}