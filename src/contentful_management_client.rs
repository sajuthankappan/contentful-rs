use crate::http_client;
use crate::query_builder::QueryBuilder;
use serde::{Deserialize, Serialize};
use serde_json::Value;

pub struct ContentfulManagementClient {
    management_api_access_token: String,
    space_id: String,
    base_url: String,
}

impl ContentfulManagementClient {
    pub fn new(management_api_access_token: &str, space_id: &str) -> ContentfulManagementClient {
        ContentfulManagementClient {
            base_url: "https://api.contentful.com/spaces".into(),
            management_api_access_token: management_api_access_token.into(),
            space_id: space_id.into(),
        }
    }

    pub async fn get_entry(&self, entry_id: &str) -> Result<Value, Box<dyn std::error::Error>>
    {
        let json = self.get_entry_json_value(entry_id).await?;
        Ok(json)
        //let entry_value = json.get_mut("fields").unwrap();
        //let entry_string = entry_value.to_string();
        //let entry = serde_json::from_str::<Value>(&entry_string.as_str())?;
        //Ok(entry)
    }

    pub async fn create_entry_json_value(&self, content_type_id: &str, entry: &Value) -> Result<Value, Box<dyn std::error::Error>> {
        let environment = "master"; 
        let url = format!(
            "{base_url}/{space_id}/environments/{environment}/entries",
            base_url = &self.base_url,
            space_id = &self.space_id,
            environment = &environment,
        );
        let mut json = http_client::post::<Value, Value>(&url, content_type_id, &self.management_api_access_token, entry).await?;
        let entry_created_fields = json.get_mut("fields").unwrap();
        let entry_created_string = entry_created_fields.to_string();
        let entry_created = serde_json::from_str::<Value>(&entry_created_string.as_str())?;
        
        Ok(entry_created)
    }

    async fn get_entry_json_value(
        &self,
        entry_id: &str,
    ) -> Result<Value, Box<dyn std::error::Error>> {
        let environment = "master";
        let url = format!(
            "{base_url}/{space_id}/environments/{environment}/entries/{entry_id}",
            base_url = &self.base_url,
            space_id = &self.space_id,
            environment = &environment,
            entry_id = &entry_id
        );
        let json_value = http_client::get::<Value>(&url, &self.management_api_access_token).await?;
        Ok(json_value)
    }
}