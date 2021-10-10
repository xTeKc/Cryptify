use notify::*;
use ureq;


fn req()-> Result<(), ureq::Error> {
    let req: String = ureq::get("https://api.coingecko.com/api/v3/simple/price?ids=monero&vs_currencies=usd")
    .set("Example-Header", "header value")
    .call()?
    .into_string()?;
    println!("{:?}", req);
  Ok(())
}

fn main() {
    let r = req();
    println!("{:?}", r);
}




