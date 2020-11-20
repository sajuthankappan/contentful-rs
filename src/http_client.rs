use reqwest;
use reqwest::StatusCode;
use serde::{Deserialize, Serialize};
use serde_json::Value;

pub(crate) async fn get<T>(
    url: &String,
    bearer_token: &String,
) -> Result<T, Box<dyn std::error::Error>>
where
    for<'a> T: Serialize + Deserialize<'a>,
{
    let client = reqwest::Client::new();
    let resp = client.get(url).bearer_auth(&bearer_token).send().await?;

    if resp.status() == StatusCode::OK {
        let json = resp.json::<T>().await?;
        Ok(json)
    } else {
        todo!("handle response status not 200");
    }
}

pub(crate) async fn post(
    url: &str,
    bearer_token: &str,
    content_type_id: &str,
    data: &Value,
) -> Result<Value, Box<dyn std::error::Error>>
{
    let client = reqwest::Client::new();
    let resp = client
        .post(url)
        .bearer_auth(&bearer_token)
        .header("X-Contentful-Content-Type", content_type_id)
        .json(&data)
        .send()
        .await?;

    if resp.status() == StatusCode::OK || resp.status() == StatusCode::CREATED {
        let json = resp.json::<Value>().await?;
        Ok(json)
    } else {
        dbg!(resp);
        todo!("handle response status not 200");
    }
}
