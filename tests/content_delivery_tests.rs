use contentful::{models::SystemProperties, ContentfulClient, QueryBuilder};
use dotenv;
use serde::{Deserialize, Serialize};

#[tokio::test]
async fn get_entry_works() {
    setup();
    let access_token = std::env::var("CONTENTFUL_ACCESS_TOKEN").unwrap();
    let space_id = std::env::var("CONTENTFUL_SPACE_ID").unwrap();
    let contentful_client = ContentfulClient::new(access_token.as_str(), space_id.as_str());
    let expected_name = "Saju";
    let entry_id = "3YrHEsZ9iUsEQOu6IQsI6k";
    let actual = contentful_client
        .get_entry::<SimplePerson>(&entry_id.to_string())
        .await
        .unwrap()
        .unwrap();
    dbg!(&actual);
    let actual_name = actual.name.as_str();
    assert_eq!(actual_name, expected_name);
    let created_at = actual.sys.created_at();
    dbg!(created_at);
}

#[tokio::test]
async fn get_entry_json_value_works() {
    setup();
    let access_token = std::env::var("CONTENTFUL_ACCESS_TOKEN").unwrap();
    let space_id = std::env::var("CONTENTFUL_SPACE_ID").unwrap();
    let contentful_client = ContentfulClient::new(access_token.as_str(), space_id.as_str());
    let expected_name = "Saju";
    let entry_id = "3YrHEsZ9iUsEQOu6IQsI6k".to_string();
    let actual = contentful_client
        .get_entry_json_value(&entry_id)
        .await
        .unwrap()
        .unwrap();
    let actual_json_str = serde_json::to_string(&actual).unwrap();
    dbg!(actual_json_str);
    let actual_name = actual["fields"]["name"].as_str().unwrap();
    assert_eq!(actual_name, expected_name);
}

#[tokio::test]
async fn get_entries_by_query_string_works() {
    setup();
    let access_token = std::env::var("CONTENTFUL_ACCESS_TOKEN").unwrap();
    let space_id = std::env::var("CONTENTFUL_SPACE_ID").unwrap();
    let contentful_client = ContentfulClient::new(access_token.as_str(), space_id.as_str());
    let name = "Saju";
    let query_string = format!("?content_type=person&fields.name={}", &name);
    let actual = contentful_client
        .get_entries_by_query_string::<Person>(query_string.as_str())
        .await
        .unwrap();
    dbg!(&actual);
    let actual_name = actual[0].clone().name;
    assert_eq!(actual_name, name);
}

#[tokio::test]
async fn get_entries_by_type_works() {
    setup();
    let access_token = std::env::var("CONTENTFUL_ACCESS_TOKEN").unwrap();
    let space_id = std::env::var("CONTENTFUL_SPACE_ID").unwrap();
    let contentful_client = ContentfulClient::new(access_token.as_str(), space_id.as_str());
    let name = "Saju";
    let query_builder = QueryBuilder::new()
        .field_equals("fields.name", name)
        .include(2);
    let actual = contentful_client
        .get_entries_by_type::<Person>("person", Some(query_builder))
        .await
        .unwrap();
    dbg!(&actual);
    let actual_name = actual[0].clone().name;
    assert_eq!(actual_name, name);
}

fn setup() {
    dotenv::dotenv().ok();
    let _ = env_logger::builder().is_test(true).try_init();
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct SimplePerson {
    name: String,
    title: String,
    short_bio: Option<String>,
    sys: SystemProperties,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct Person {
    name: String,
    title: String,
    short_bio: Option<String>,
    favorite_product: Option<Product>,
    interested_products: Option<Vec<Product>>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct Product {
    title: String,
    related_trainings: Option<Vec<TrainingPlan>>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct TrainingPlan {
    topic: String,
    slug: String,
}
