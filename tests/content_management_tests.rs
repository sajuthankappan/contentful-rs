use contentful::{ContentfulManagementClient, QueryBuilder};
use dotenv;
use serde::{Deserialize, Serialize};
use serde_json::json;

#[tokio::test]
async fn get_entry_works() {
    setup();
    let access_token = std::env::var("CONTENTFUL_MANAGEMENT_TOKEN").unwrap();
    let space_id = std::env::var("CONTENTFUL_SPACE_ID").unwrap();
    let contentful_client = ContentfulManagementClient::new(access_token.as_str(), space_id.as_str());
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
async fn create_entry_works() {
    setup();
    let access_token = std::env::var("CONTENTFUL_MANAGEMENT_TOKEN").unwrap();
    let space_id = std::env::var("CONTENTFUL_SPACE_ID").unwrap();
    let contentful_client = ContentfulManagementClient::new(access_token.as_str(), space_id.as_str());
    let expected_name = "Saju-rs";
    let entry = json!({
        "name": {
            "en-US": "Saju-rs",
        },
        "title": {
            "en-US": "Mr",
        },
    });
    let entry_created = contentful_client
        .create_entry_json_value( "person", &entry)
        .await
        .unwrap();
    dbg!(&entry_created);
    let actual_name = entry_created["fields"]["name"]["en-US"].clone();
    assert_eq!(actual_name, expected_name);
}

fn setup() {
    dotenv::dotenv().ok();
    let _ = env_logger::builder().is_test(true).try_init();
}
