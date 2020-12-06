use reqwest;
use reqwest::StatusCode;
use serde::de::DeserializeOwned;
use serde_json::Value;

pub(crate) async fn get<T>(
    url: &String,
    bearer_token: &String,
) -> Result<Option<T>, Box<dyn std::error::Error>>
where
    T: DeserializeOwned,
{
    let client = reqwest::Client::new();
    let resp = client.get(url).bearer_auth(&bearer_token).send().await?;

    if resp.status() == StatusCode::OK {
        let json = resp.json::<T>().await?;
        Ok(Some(json))
    } else if resp.status() == StatusCode::NOT_FOUND {
        Ok(None)
    } else {
        log::warn!("{:?}", &resp);
        log::warn!("{:?}", &resp.text().await?);
        todo!("handle response status not < 300");
    }
}

pub(crate) async fn post(
    url: &str,
    bearer_token: &str,
    content_type_id: &str,
    data: &Value,
) -> Result<Value, Box<dyn std::error::Error>> {
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
        log::warn!("{:?}", &resp);
        log::warn!("{:?}", &resp.text().await?);
        todo!("handle response status not < 300");
    }
}

pub(crate) async fn put(
    url: &str,
    bearer_token: &str,
    version: i32,
    content_type_id: &str,
    data: &Value,
) -> Result<Value, Box<dyn std::error::Error>> {
    let client = reqwest::Client::new();
    let resp = client
        .put(url)
        .bearer_auth(&bearer_token)
        .header("X-Contentful-Content-Type", content_type_id)
        .header("X-Contentful-Version", version)
        .json(&data)
        .send()
        .await?;

    if resp.status() == StatusCode::OK {
        let json = resp.json::<Value>().await?;
        Ok(json)
    } else {
        log::warn!("{:?}", &resp);
        log::warn!("{:?}", &resp.text().await?);
        todo!("handle response status not < 300");
    }
}
