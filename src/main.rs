// use notify::*;
// use ureq;
// use std::error::Error;
// use serde::Deserialize;

// #[derive(Deserialize, Debug)]
// pub struct Cryptos {
//   cryptos: Vec<Crypto>
// }

// #[derive(Deserialize, Debug)]
// pub struct Crypto {
//   // name: String,
//   symbol: String,
//   marketcap: u32
// }

// fn crypto_req(req: String) -> Result<Cryptos, dyn Error> {
//     let resp = ureq::get(req).call()?.into_string()?;
//     let cryptos: Cryptos = serde_json::from_str(&resp)?;

//     dbg!(cryptos);
  
//     // todo!()
//     Ok(())
//   }

// //   enum Result<T, E> {
// //     Ok(T),
// //     Err(E),
// //  }

// fn main() {
//     let req = "https://api.coingecko.com/api/v3/simple/price?ids=monero&vs_currencies=usd";
//     let cryptos = crypto_req(req);
//     dbg!(cryptos);
// }

use notify::*;
use ureq;

fn url_req() {
    let _req = format!("https://api.coingecko.com/api/v3/simple/price?ids=monero&vs_currencies=usd");
}

fn main(){
  url_req();
}