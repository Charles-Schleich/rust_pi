use reqwest::*;
use std::collections::HashMap;

fn main() {
    println!("Hello, world!");
    pull_data();
}



fn pull_data() -> Result<()> {
    let resp = reqwest::blocking::get("https://httpbin.org/ip")?
        .json::<HashMap<String, String>>()?;
    println!("{:#?}", resp);
    Ok(())
}