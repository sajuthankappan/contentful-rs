use reqwest;
use reqwest::StatusCode;
use serde::{Deserialize, Serialize};

pub async fn get<T>(url: &String, bearer_token: &String) -> Result<T, Box<dyn std::error::Error>>
where
    for<'a> T: Serialize + Deserialize<'a>,
{
    let client = reqwest::Client::new();
    let resp = client.get(url).bearer_auth(&bearer_token).send().await?;
    log::debug!("{:?}", &resp);

    if resp.status() == StatusCode::OK {
        let json = resp.json::<T>().await?;
        Ok(json)
    } else {
        todo!("handle response status not 200");
    }
}
