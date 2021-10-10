use notify::*;
use http::{Request, Response};


fn req() {
    let request = Request::builder()
    .uri("https://api.coingecko.com/api/v3/simple/price?ids=monero&vs_currencies=usd")
    .header("User-Agent", "awesome/1.0")
    .body(())
    .unwrap();
    }

fn main() {
    req();
}