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

    match resp.status() {
        StatusCode::OK => {
            let json = resp.json::<T>().await?;
            Ok(Some(json))
        }
        StatusCode::NOT_FOUND => Ok(None),
        _ => {
            log::warn!("{:?}", &resp);
            log::warn!("{:?}", &resp.text().await?);
            todo!("handle response status not < 300");
        }
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

    match resp.status() {
        StatusCode::OK | StatusCode::CREATED => {
            let json = resp.json::<Value>().await?;
            Ok(json)
        }
        _ => {
            log::warn!("{:?}", &resp);
            log::warn!("{:?}", &resp.text().await?);
            todo!("handle response status not < 300");
        }
    }
}

pub(crate) async fn put(
    url: &str,
    bearer_token: &str,
    version: &Option<i32>,
    content_type_id: &str,
    data: &Value,
) -> Result<Value, Box<dyn std::error::Error>> {
    let client = reqwest::Client::new();
    let mut builder = client
        .put(url)
        .bearer_auth(&bearer_token)
        .header("X-Contentful-Content-Type", content_type_id)
        .json(&data);

    if let Some(version) = version {
        builder = builder.header("X-Contentful-Version", version.clone());
    }

    let resp = builder.send().await?;

    match resp.status() {
        StatusCode::OK | StatusCode::CREATED => {
            let json = resp.json::<Value>().await?;
            Ok(json)
        }
        _ => {
            log::warn!("{:?}", &resp);
            log::warn!("{:?}", &resp.text().await?);
            todo!("handle response status not < 300");
        }
    }
}
