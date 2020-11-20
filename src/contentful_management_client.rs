use crate::http_client;
use serde::{Deserialize, Serialize};
use serde_json::json;
use serde_json::Value;

pub struct ContentfulManagementClient {
    management_api_access_token: String,
    space_id: String,
    base_url: String,
    environment_id: String,
}

impl ContentfulManagementClient {
    pub fn new(management_api_access_token: &str, space_id: &str) -> ContentfulManagementClient {
        let environment_id = "master".into();
        ContentfulManagementClient {
            base_url: "https://api.contentful.com/spaces".into(),
            management_api_access_token: management_api_access_token.into(),
            space_id: space_id.into(),
            environment_id,
        }
    }

    pub async fn get_entry(&self, entry_id: &str) -> Result<Value, Box<dyn std::error::Error>> {
        let json = self.get_entry_json_value(entry_id).await?;
        Ok(json)
        //let entry_value = json.get_mut("fields").unwrap();
        //let entry_string = entry_value.to_string();
        //let entry = serde_json::from_str::<Value>(&entry_string.as_str())?;
        //Ok(entry)
    }

    pub async fn create_entry_from_json_value<T>(
        &self,
        content_type_id: &str,
        entry: &Value,
    ) -> Result<T, Box<dyn std::error::Error>>
    where
        for<'a> T: Serialize + Deserialize<'a>,
    {
        let url = format!(
            "{base_url}/{space_id}/environments/{environment_id}/entries",
            base_url = &self.base_url,
            space_id = &self.space_id,
            environment_id = &self.environment_id,
        );
        let mut json = http_client::post(
            &url,
            &self.management_api_access_token,
            content_type_id,
            entry,
        )
        .await?;
        let entry_created_fields = json.get_mut("fields").unwrap();
        let entry_created_string = entry_created_fields.to_string();
        let entry_created = serde_json::from_str::<T>(&entry_created_string.as_str())?;

        Ok(entry_created)
    }

    pub async fn create_entry<T>(
        &self,
        content_type_id: &str,
        entry: &T,
    ) -> Result<T, Box<dyn std::error::Error>>
    where
        for<'a> T: Serialize + Deserialize<'a>,
    {
        let entry_json_value = json!({ "fields": entry });
        self.create_entry_from_json_value::<T>(content_type_id, &entry_json_value)
            .await
    }

    pub async fn create_entry_for_locale<T>(
        &self,
        content_type_id: &str,
        locale: &str,
        entry: &T,
    ) -> Result<T, Box<dyn std::error::Error>>
    where
        for<'a> T: Serialize + Deserialize<'a>,
    {
        let mut entry_json_value = json!(entry);
        let mut entry_to_create = serde_json::Map::new();
        if entry_json_value.is_object() {
            let entry_object = entry_json_value.as_object_mut().unwrap();
            for (field_name, field_value) in entry_object {
                if field_value.is_object() {
                    todo!();
                } else if field_value.is_array() {
                    todo!();
                } else {
                    entry_to_create.insert(field_name.into(), json!({ locale: field_value }));
                }
            }
        } else {
            unimplemented!();
        }
        dbg!(&entry_to_create);
        let mut created_entry_json_value = self.create_entry::<Value>(content_type_id, &json!(entry_to_create))
            .await?;
        dbg!(&created_entry_json_value);
        let mut entry_created_map = serde_json::Map::new();
        if created_entry_json_value.is_object() {
            let entry_object = created_entry_json_value.as_object_mut().unwrap();
            for (field_name, field_value) in entry_object {
                if field_value.is_object() {
                    entry_created_map.insert(field_name.into(), field_value.get(locale).unwrap().clone());
                } else if field_value.is_array() {
                    todo!();
                } else {
                    todo!();
                }
            }
        } else {
            todo!();
        }
        let entry_created_string = json!(entry_created_map).to_string();
        let created_entry = serde_json::from_str::<T>(&entry_created_string.as_str())?;
        Ok(created_entry)
    }

    async fn get_entry_json_value(
        &self,
        entry_id: &str,
    ) -> Result<Value, Box<dyn std::error::Error>> {
        let url = format!(
            "{base_url}/{space_id}/environments/{environment_id}/entries/{entry_id}",
            base_url = &self.base_url,
            space_id = &self.space_id,
            environment_id = &self.environment_id,
            entry_id = &entry_id
        );
        let json_value = http_client::get::<Value>(&url, &self.management_api_access_token).await?;
        Ok(json_value)
    }
}
