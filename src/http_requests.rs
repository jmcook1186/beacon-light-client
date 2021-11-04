use std::format;
use std::vec;
use std::error::Error;
use serde_json::{Value};
use reqwest::{
  header::{HeaderMap, HeaderValue}};




pub fn get_request_auth_header(api_key: &str)-> Result<HeaderMap, Box<dyn Error>>{

  let mut _headers = HeaderMap::new();
  let header_value = HeaderValue::from_str(&format!("Basic {}", api_key))
  .map_err(|e| {
    format!("secret is invalid as a header value: {}", e)
      })?;

  _headers.insert("Authorization", header_value);

  Ok(_headers)

}



#[tokio::main]
pub async fn generic_request(api_key: &str, endpoint: &str, node_id: &str)-> Result<serde_json::Value,reqwest::Error>{
  
  let prefix: String = format!("http://localhost:{}/eth/v1/",node_id);
  let url: String = prefix+endpoint;
  let client = reqwest::Client::new();
  let _headers: HeaderMap = get_request_auth_header(api_key).unwrap();

  let response = 
    client.get(&url).headers(_headers).send().await?;
  
  println!("{}",&url);
  println!("Status: {}", &response.status());

  let body = response.text().await.unwrap();

  let result: serde_json::Value =
  serde_json::from_str(&body).expect("JSON was not well-formatted");

  // println!("{}", result["data"][1]["validator"]["pubkey"].to_string());
  
  Ok(result)

}
