use contentful_fork::{
    models::{Entry, SystemProperties},
    ContentfulManagementClient,
};
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
        .get_entry(&entry_id)
        .await
        .unwrap()
        .unwrap();
    dbg!(&actual);
    let actual_name = actual.fields["name"]["en-US"].clone();
    assert_eq!(actual_name, expected_name);
}

#[tokio::test]
async fn get_entry_for_locale_works() {
    setup();
    let access_token = std::env::var("CONTENTFUL_MANAGEMENT_TOKEN").unwrap();
    let space_id = std::env::var("CONTENTFUL_SPACE_ID").unwrap();
    let contentful_client =
        ContentfulManagementClient::new(access_token.as_str(), space_id.as_str());
    let expected_name = "Saju";
    let entry_id = "3YrHEsZ9iUsEQOu6IQsI6k";
    let locale = "en-US";
    let actual = contentful_client
        .get_entry_for_locale::<Person>(&entry_id, locale)
        .await
        .unwrap()
        .unwrap();
    dbg!(&actual);
    let actual_name = actual.fields.name;
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
        .create_entry_from_json::<Value>(&entry, "person")
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
        .create_entry::<PersonMap>(&entry, "person")
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
        .create_entry_for_locale::<Person>(&entry, "person", "en-US")
        .await
        .unwrap();
    dbg!(&entry_created);
    let actual_name = entry_created.name;
    assert_eq!(actual_name, expected_name);
}

#[tokio::test]
async fn create_or_update_entry_for_locale_works() {
    setup();
    let access_token = std::env::var("CONTENTFUL_MANAGEMENT_TOKEN").unwrap();
    let space_id = std::env::var("CONTENTFUL_SPACE_ID").unwrap();
    let contentful_client =
        ContentfulManagementClient::new(access_token.as_str(), space_id.as_str());
    let expected_name = "Saju-rs-3";
    let name = "Saju-rs-3".into();
    let title = "Mr".into();

    let person = Person { name, title };
    let sys = SystemProperties::with_version("3zEzRLcj41sahE9SuTdRsU".into(), 13);
    let entry = Entry::new(person, sys);
    //let entry = ContentfulManagementClient::get_entry(entry_id);
    let entry_updated = contentful_client
        .create_or_update_entry_for_locale(&entry, "3zEzRLcj41sahE9SuTdRsU", "en-US", "person")
        .await
        .unwrap();
    dbg!(&entry_updated);
    let actual_name = entry_updated.fields.name;
    assert_eq!(actual_name, expected_name);
}

#[tokio::test]
async fn get_entry_and_update_works() {
    setup();
    let access_token = std::env::var("CONTENTFUL_MANAGEMENT_TOKEN").unwrap();
    let space_id = std::env::var("CONTENTFUL_SPACE_ID").unwrap();
    let contentful_client =
        ContentfulManagementClient::new(access_token.as_str(), space_id.as_str());
    let expected_name = "Saju";
    let entry_id = "3YrHEsZ9iUsEQOu6IQsI6k";
    let locale = "en-US";
    let actual = contentful_client
        .get_entry_for_locale::<Person>(&entry_id, locale)
        .await
        .unwrap()
        .unwrap();
    dbg!(&actual);
    let actual_person = actual.clone().fields;
    let actual_name = actual_person.name.clone();
    assert_eq!(actual_name, expected_name);

    let mut new_person_entry = actual.clone();
    let mut new_person = new_person_entry.fields;
    new_person.title = "new title".into();
    new_person_entry.fields = new_person;

    let content_type_id = "person";
    let updated_person = contentful_client
        .create_or_update_entry_for_locale(&new_person_entry, entry_id, locale, content_type_id)
        .await;
    dbg!(&updated_person);
}

#[tokio::test]
async fn get_entry_and_update_for_value_works() {
    setup();
    let access_token = std::env::var("CONTENTFUL_MANAGEMENT_TOKEN").unwrap();
    let space_id = std::env::var("CONTENTFUL_SPACE_ID").unwrap();
    let contentful_client =
        ContentfulManagementClient::new(access_token.as_str(), space_id.as_str());
    let expected_name = "Saju";
    let entry_id = "3YrHEsZ9iUsEQOu6IQsI6k";
    let locale = "en-US";
    let actual = contentful_client
        .get_entry(&entry_id)
        .await
        .unwrap()
        .unwrap();
    dbg!(&actual);
    let actual_person = actual.clone().fields;
    let actual_name = actual_person["name"][locale].clone();
    assert_eq!(actual_name, expected_name);

    let mut new_person_entry = actual.clone();
    let mut new_person = new_person_entry.fields;
    new_person["title"] = json!({locale: "new title"});
    new_person_entry.fields = new_person;

    let content_type_id = "person";
    let updated_person = contentful_client
        .create_or_update_entry(&new_person_entry, entry_id, content_type_id)
        .await;
    dbg!(&updated_person);
}

#[tokio::test]
async fn get_entry_and_update_for_locale_for_value_works() {
    setup();
    let access_token = std::env::var("CONTENTFUL_MANAGEMENT_TOKEN").unwrap();
    let space_id = std::env::var("CONTENTFUL_SPACE_ID").unwrap();
    let contentful_client =
        ContentfulManagementClient::new(access_token.as_str(), space_id.as_str());
    let expected_name = "Saju";
    let entry_id = "3YrHEsZ9iUsEQOu6IQsI6k";
    let locale = "en-US";
    let actual = contentful_client
        .get_entry_for_locale::<Value>(&entry_id, locale)
        .await
        .unwrap()
        .unwrap();
    dbg!(&actual);
    let actual_person = actual.clone().fields;
    let actual_name = actual_person["name"].clone();
    assert_eq!(actual_name, expected_name);

    let mut new_person_entry = actual.clone();
    let mut new_person = new_person_entry.fields;
    new_person["title"] = "new title".into();
    new_person_entry.fields = new_person;

    let content_type_id = "person";
    let updated_person = contentful_client
        .create_or_update_entry_for_locale(&new_person_entry, entry_id, locale, content_type_id)
        .await;
    dbg!(&updated_person);
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
