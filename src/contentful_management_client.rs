use crate::{http_client, models::Entry};
use serde::{de::DeserializeOwned, Serialize};
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

    pub async fn get_entry(
        &self,
        entry_id: &str,
    ) -> Result<Option<Entry<Value>>, Box<dyn std::error::Error>> {
        let url = format!(
            "{base_url}/{space_id}/environments/{environment_id}/entries/{entry_id}",
            base_url = &self.base_url,
            space_id = &self.space_id,
            environment_id = &self.environment_id,
            entry_id = &entry_id
        );
        let entry =
            http_client::get::<Entry<Value>>(&url, &self.management_api_access_token).await?;
        Ok(entry)
    }

    pub async fn get_entry_for_locale<T>(
        &self,
        entry_id: &str,
        locale: &str,
    ) -> Result<Option<Entry<T>>, Box<dyn std::error::Error>>
    where
        T: DeserializeOwned + Serialize,
    {
        let url = format!(
            "{base_url}/{space_id}/environments/{environment_id}/entries/{entry_id}",
            base_url = &self.base_url,
            space_id = &self.space_id,
            environment_id = &self.environment_id,
            entry_id = &entry_id
        );
        if let Some(entry_json) =
            http_client::get::<Entry<Value>>(&url, &self.management_api_access_token).await?
        {
            let entry_typed = helpers::convert_json_object_to_typed_entry(entry_json.fields().clone(), locale)?;
            let entry = Entry::new(entry_typed, entry_json.sys().clone());
            Ok(Some(entry))
        } else {
            Ok(None)
        }
    }

    pub async fn create_entry_from_json<T>(
        &self,
        entry: &Value,
        content_type_id: &str,
    ) -> Result<T, Box<dyn std::error::Error>>
    where
        T: DeserializeOwned,
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
        entry: &T,
        content_type_id: &str,
    ) -> Result<T, Box<dyn std::error::Error>>
    where
        T: DeserializeOwned + Serialize,
    {
        let entry_json = json!({ "fields": entry });
        self.create_entry_from_json::<T>(&entry_json, content_type_id)
            .await
    }

    pub async fn create_entry_for_locale<T>(
        &self,
        entry: &T,
        content_type_id: &str,
        locale: &str,
    ) -> Result<T, Box<dyn std::error::Error>>
    where
        T: DeserializeOwned + Serialize,
    {
        let entry_to_create = helpers::reconstruct_json_object(entry, locale)?;
        let updated_entry_json = self
            .create_entry::<Value>(&entry_to_create, content_type_id)
            .await?;
        let updated_entry =
            helpers::convert_json_object_to_typed_entry(updated_entry_json, locale)?;
        Ok(updated_entry)
    }

    pub async fn update_entry_from_json(
        &self,
        entry: &Value,
        id: &str,
        version: &i32,
        content_type_id: &str,
    ) -> Result<Value, Box<dyn std::error::Error>> {
        let url = format!(
            "{base_url}/{space_id}/environments/{environment_id}/entries/{id}",
            base_url = &self.base_url,
            space_id = &self.space_id,
            environment_id = &self.environment_id,
            id = &id,
        );
        let json = http_client::put(
            &url,
            &self.management_api_access_token,
            version.clone(),
            content_type_id,
            entry,
        )
        .await?;
        Ok(json)
    }

    pub async fn update_entry(
        &self,
        entry: &Entry<Value>,
        id: &str,
        content_type_id: &str,
    ) -> Result<Entry<Value>, Box<dyn std::error::Error>> {
        if let Some(version) = entry.sys().version() {
            let entry_updated = self
                .update_entry_from_json(&json!(entry), id, version, content_type_id)
                .await?;
            let entry_updated_string = entry_updated.to_string();
            let entry = serde_json::from_str::<Entry<Value>>(&entry_updated_string.as_str())?;
            Ok(entry)
        } else {
            todo!();
        }
    }

    pub async fn update_entry_for_locale<T>(
        &self,
        entry: &Entry<T>,
        id: &str,
        locale: &str,
        content_type_id: &str,
    ) -> Result<Entry<T>, Box<dyn std::error::Error>>
    where
        T: DeserializeOwned + Serialize,
    {
        let entry_json = helpers::reconstruct_json_object(entry.fields(), locale)?;
        let entry_to_update = Entry::new(entry_json, entry.sys().clone());
        let updated_entry_json = self
            .update_entry(&entry_to_update, id, content_type_id)
            .await?;
        let updated_entry_typed = helpers::convert_json_object_to_typed_entry(
            json!(updated_entry_json.fields()),
            locale,
        )?;
        let updated_entry = Entry::new(updated_entry_typed, updated_entry_json.sys().clone());
        Ok(updated_entry)
    }
}

mod helpers {
    use serde::{de::DeserializeOwned, Serialize};
    use serde_json::{json, Value};

    pub fn reconstruct_json_object<T>(
        entry: &T,
        locale: &str,
    ) -> Result<Value, Box<dyn std::error::Error>>
    where
        T: Serialize,
    {
        let mut entry_json = json!(entry);
        let mut fields_map = serde_json::Map::new();

        if entry_json.is_object() {
            let entry_object = entry_json.as_object_mut().unwrap();
            for (field_name, field_value) in entry_object {
                if field_value.is_object() {
                    todo!();
                } else if field_value.is_array() {
                    todo!();
                } else {
                    fields_map.insert(field_name.into(), json!({ locale: field_value }));
                }
            }
        } else {
            unimplemented!();
        }
        return Ok(json!(fields_map));
    }

    pub fn convert_json_object_to_typed_entry<T>(
        entry_json: Value,
        locale: &str,
    ) -> Result<T, Box<dyn std::error::Error>>
    where
        T: DeserializeOwned,
    {
        let mut entry_created_map = serde_json::Map::new();

        if entry_json.is_object() {
            let entry_object = entry_json.as_object().unwrap();
            for (field_name, field_value) in entry_object {
                if field_value.is_object() {
                    entry_created_map
                        .insert(field_name.into(), field_value.get(locale).unwrap().clone());
                } else if field_value.is_array() {
                    todo!();
                } else {
                    todo!();
                }
            }
        } else {
            todo!();
        }

        let entry_string = json!(entry_created_map).to_string();
        let created_entry = serde_json::from_str::<T>(&entry_string.as_str())?;
        Ok(created_entry)
    }
}
