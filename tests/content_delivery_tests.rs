use contentful::{ContentfulClient, QueryBuilder};
use dotenv;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
struct Person {
    name: String,
    title: String,
    #[serde(rename = "shortBio")]
    short_bio: Option<String>,
    #[serde(rename = "favoriteProduct")]
    favorite_product: Option<Product>,
    #[serde(rename = "interestedProducts")]
    interested_products: Option<Vec<Product>>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
struct Product {
    title: String,
}

#[tokio::test]
async fn get_entry_works() {
    setup();
    let access_token = std::env::var("CONTENTFUL_ACCESS_TOKEN").unwrap();
    let space_id = std::env::var("CONTENTFUL_SPACE_ID").unwrap();
    let contentful_client = ContentfulClient::new(access_token, space_id);
    let expected_name = "Saju".to_string();
    let entry_id = "3YrHEsZ9iUsEQOu6IQsI6k".to_string();
    let actual = contentful_client
        .get_entry::<Person>(entry_id)
        .await
        .unwrap();
    dbg!(&actual);
    let actual_name = actual.name;
    assert_eq!(actual_name, expected_name);
}

#[tokio::test]
async fn get_entries_by_query_string_works() {
    setup();
    let access_token = std::env::var("CONTENTFUL_ACCESS_TOKEN").unwrap();
    let space_id = std::env::var("CONTENTFUL_SPACE_ID").unwrap();
    let contentful_client = ContentfulClient::new(access_token, space_id);
    let expected_name = "Saju".to_string();
    let query_string = format!("?content_type=person&fields.name={}", &expected_name);
    let actual = contentful_client
        .get_entries_by_query_string::<Person>(Some(query_string))
        .await
        .unwrap();
    dbg!(&actual);
    let actual_name = actual[0].clone().name;
    assert_eq!(actual_name, expected_name);
}

#[tokio::test]
async fn get_entries_by_type_works() {
    setup();
    let access_token = std::env::var("CONTENTFUL_ACCESS_TOKEN").unwrap();
    let space_id = std::env::var("CONTENTFUL_SPACE_ID").unwrap();
    let contentful_client = ContentfulClient::new(access_token, space_id);
    let expected_name = "Saju".to_string();
    let query_builder = QueryBuilder::new().field_equals("fields.name".to_string(), expected_name.clone());
    let actual = contentful_client
        .get_entries_by_type::<Person>("person".to_string(), Some(query_builder))
        .await
        .unwrap();
    dbg!(&actual);
    let actual_name = actual[0].clone().name;
    assert_eq!(actual_name, expected_name);
}

fn setup() {
    dotenv::dotenv().ok();
    let _ = env_logger::builder().is_test(true).try_init();
}
