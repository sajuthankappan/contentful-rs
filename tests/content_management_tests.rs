use contentful::{ContentfulManagementClient};
use dotenv;
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use std::collections::HashMap;

#[tokio::test]
async fn get_entry_works() {
    setup();
    let access_token = std::env::var("CONTENTFUL_MANAGEMENT_TOKEN").unwrap();
    let space_id = std::env::var("CONTENTFUL_SPACE_ID").unwrap();
    let contentful_client =
        ContentfulManagementClient::new(access_token.as_str(), space_id.as_str());
    let expected_name = "Saju";
    let entry_id = "3YrHEsZ9iUsEQOu6IQsI6k";
    let actual = contentful_client
        .get_entry(&entry_id.to_string())
        .await
        .unwrap();
    dbg!(&actual);
    let actual_name = actual["fields"]["name"]["en-US"].clone();
    assert_eq!(actual_name, expected_name);
}

#[tokio::test]
async fn create_entry_from_json_value_works() {
    setup();
    let access_token = std::env::var("CONTENTFUL_MANAGEMENT_TOKEN").unwrap();
    let space_id = std::env::var("CONTENTFUL_SPACE_ID").unwrap();
    let contentful_client =
        ContentfulManagementClient::new(access_token.as_str(), space_id.as_str());
    let expected_name = "Saju-rs";
    let entry = json!({"fields": {
        "name": {
            "en-US": "Saju-rs",
        },
        "title": {
            "en-US": "Mr",
        },
    }});
    let entry_created = contentful_client
        .create_entry_from_json_value::<Value>("person", &entry)
        .await
        .unwrap();
    dbg!(&entry_created);
    let actual_name = entry_created["name"]["en-US"].clone();
    assert_eq!(actual_name, expected_name);
}

#[tokio::test]
async fn create_entry_works() {
    setup();
    let access_token = std::env::var("CONTENTFUL_MANAGEMENT_TOKEN").unwrap();
    let space_id = std::env::var("CONTENTFUL_SPACE_ID").unwrap();
    let contentful_client =
        ContentfulManagementClient::new(access_token.as_str(), space_id.as_str());
    let expected_name = "Saju-rs-2";
    let mut name = HashMap::new();
    name.insert("en-US".into(), "Saju-rs-2".into());
    let mut title = HashMap::new();
    title.insert("en-US".into(), "Mr".into());

    let entry = PersonMap { name, title };
    let entry_created = contentful_client
        .create_entry::<PersonMap>("person", &entry)
        .await
        .unwrap();
    dbg!(&entry_created);
    let actual_name = entry_created.name["en-US"].clone();
    assert_eq!(actual_name, expected_name);
}

#[tokio::test]
async fn create_entry_for_locale_works() {
    setup();
    let access_token = std::env::var("CONTENTFUL_MANAGEMENT_TOKEN").unwrap();
    let space_id = std::env::var("CONTENTFUL_SPACE_ID").unwrap();
    let contentful_client =
        ContentfulManagementClient::new(access_token.as_str(), space_id.as_str());
    let expected_name = "Saju-rs-3";
    let name = "Saju-rs-3".into();
    let title = "Mr".into();

    let entry = Person { name, title };
    let entry_created = contentful_client
        .create_entry_for_locale::<Person>("person", "en-US", &entry)
        .await
        .unwrap();
    dbg!(&entry_created);
    let actual_name = entry_created.name;
    assert_eq!(actual_name, expected_name);
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct PersonMap {
    pub name: HashMap<String, String>,
    pub title: HashMap<String, String>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct Person {
    pub name: String,
    pub title: String,
}

fn setup() {
    dotenv::dotenv().ok();
    let _ = env_logger::builder().is_test(true).try_init();
}
