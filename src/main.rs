use notify::*;


fn req() -> Result<(), ureq::Error> {
    let body: String = ureq::get("https://api.coingecko.com/api/v3/simple/price?ids=monero&vs_currencies=usd")
        .set("Example-Header", "header value")
        .call()?
        .into_string()?;
    Ok(())
}

fn main() {
    req();
}