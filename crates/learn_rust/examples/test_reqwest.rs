use std::collections::HashMap;

use reqwest::{ClientBuilder, Error, redirect::Policy};

#[tokio::main]
async fn main() -> Result<(), Error> {
  let resp = reqwest::get("https://httpbin.org/ip")
    .await?
    .json::<HashMap<String, String>>()
    .await?;
  println!("{resp:#?}");

  let redir_policy = Policy::limited(10);
  let client = ClientBuilder::new().redirect(redir_policy).build()?;
  let resp = client.get("https://httpbin.org/redirect/5").send().await?;

  let resp_str: HashMap<String, serde_json::Value> = resp.json().await?;

  println!("{:?}", resp_str);
  Ok(())
}
