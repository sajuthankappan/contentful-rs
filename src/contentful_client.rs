use crate::query_builder::QueryBuilder;
use crate::{http_client, models::Entry};
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};

pub struct ContentfulClient {
    delivery_api_access_token: String,
    space_id: String,
    base_url: String,
}

impl ContentfulClient {
    pub fn new(delivery_api_access_token: &str, space_id: &str) -> ContentfulClient {
        ContentfulClient {
            base_url: "https://cdn.contentful.com/spaces".to_string(),
            delivery_api_access_token: delivery_api_access_token.to_string(),
            space_id: space_id.to_string(),
        }
    }

    pub async fn get_entry<T>(
        &self,
        entry_id: &str,
    ) -> Result<Option<T>, Box<dyn std::error::Error>>
    where
        for<'a> T: Serialize + Deserialize<'a>,
    {
        if let Some(entry) = self.get_contentful_entry(entry_id).await? {
            let mut entry_json_value = entry.fields().clone();
            entry_json_value["sys"] = json!(entry.sys());
            let entry_string = entry_json_value.to_string();
            let entry = serde_json::from_str::<T>(&entry_string.as_str())?;
            Ok(Some(entry))
        } else {
            Ok(None)
        }
    }

    pub async fn get_contentful_entry(
        &self,
        entry_id: &str,
    ) -> Result<Option<Entry<Value>>, Box<dyn std::error::Error>> {
        let environment = "master";
        let url = format!(
            "{base_url}/{space_id}/environments/{environment}/entries/{entry_id}",
            base_url = &self.base_url,
            space_id = &self.space_id,
            environment = &environment,
            entry_id = &entry_id
        );
        let json_value =
            http_client::get::<Entry<Value>>(&url, &self.delivery_api_access_token).await?;
        Ok(json_value)
    }

    pub async fn get_entry_json_value(
        &self,
        entry_id: &str,
    ) -> Result<Option<Value>, Box<dyn std::error::Error>> {
        let environment = "master";
        let url = format!(
            "{base_url}/{space_id}/environments/{environment}/entries/{entry_id}",
            base_url = &self.base_url,
            space_id = &self.space_id,
            environment = &environment,
            entry_id = &entry_id
        );
        let json_value = http_client::get::<Value>(&url, &self.delivery_api_access_token).await?;
        Ok(json_value)
    }

    pub async fn get_entries<T>(
        &self,
        query_builder: Option<QueryBuilder>,
    ) -> Result<Vec<T>, Box<dyn std::error::Error>>
    where
        for<'a> T: Serialize + Deserialize<'a>,
    {
        let query_string = if let Some(query_builder) = query_builder {
            query_builder.build()
        } else {
            "".to_string()
        };

        self.get_entries_by_query_string::<T>(query_string.as_str())
            .await
    }

    pub async fn get_entries_by_query_string<T>(
        &self,
        query_string: &str,
    ) -> Result<Vec<T>, Box<dyn std::error::Error>>
    where
        for<'a> T: Serialize + Deserialize<'a>,
    {
        log::debug!("query_string: {:?}", &query_string);
        let environment = "master";
        let url = format!(
            "{base_url}/{space_id}/environments/{environment}/entries{query_string}",
            base_url = &self.base_url,
            space_id = &self.space_id,
            environment = &environment,
            query_string = &query_string
        );
        if let Some(mut json) =
            http_client::get::<Value>(&url, &self.delivery_api_access_token).await?
        {
            let includes = json.get("includes").unwrap().clone(); // TODO: Check if clone can be avoided
            let items = json.get_mut("items").unwrap();

            if items.is_array() {
                self.resolve_array(items, &includes)?;
                let ar_string = items.to_string();
                let entries = serde_json::from_str::<Vec<T>>(&ar_string.as_str())?;
                Ok(entries)
            } else {
                unimplemented!();
            }
        } else {
            unimplemented!();
        }
    }

    pub async fn get_entries_by_type<T>(
        &self,
        content_type: &str,
        query_builder: Option<QueryBuilder>,
    ) -> Result<Vec<T>, Box<dyn std::error::Error>>
    where
        for<'a> T: Serialize + Deserialize<'a>,
    {
        let query_builder = query_builder
            .unwrap_or(QueryBuilder::new())
            .content_type_is(content_type);

        self.get_entries(Some(query_builder)).await
        //self.get_entries_by_query_string::<T>(Some(new_query_string)).await
    }

    fn resolve_array(
        &self,
        value: &mut Value,
        includes: &Value,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let items = value.as_array_mut().unwrap();
        for item in items {
            if item.is_object() {
                self.resolve_object(item, &includes)?;
            } else {
                unimplemented!();
            }
        }
        Ok(())
    }

    fn resolve_object(
        &self,
        value: &mut Value,
        includes: &Value,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let sys_type = value["sys"]["type"].clone();
        if sys_type == "Entry" {
            self.resolve_entry(value, &includes)?;
        //*value = value["fields"].clone();
        } else if sys_type == "Link" {
            self.resolve_link(value, &includes)?;
        //*value = value["fields"].clone();
        } else {
            let node_type = value["nodeType"].clone();
            if node_type == "document" {
                log::warn!("TODO: richtext");
            } else {
                unimplemented!("{} - {} not implemented", &sys_type, &node_type);
            }
        }
        Ok(())
    }

    fn resolve_entry(
        &self,
        value: &mut Value,
        includes: &Value,
    ) -> Result<(), Box<dyn std::error::Error>> {
        if let Some(fields) = value.get_mut("fields") {
            if fields.is_object() {
                let entry_object = fields.as_object_mut().unwrap();
                for (_field_name, field_value) in entry_object {
                    if field_value.is_object() {
                        self.resolve_object(field_value, &includes)?;
                    } else if field_value.is_array() {
                        self.resolve_array(field_value, &includes)?;
                    } else {
                        // Regular string, number, etc, values. No need to do anything.
                    }
                }
                *value = fields.clone();
            } else {
                unimplemented!();
            }
        } else {
            unimplemented!();
        }

        Ok(())
    }

    fn resolve_link(
        &self,
        value: &mut Value,
        includes: &Value,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let link_type = value["sys"]["linkType"].clone();
        let link_id = value["sys"]["id"].clone();
        if link_type == "Entry" {
            let included_entries = includes["Entry"].as_array().unwrap();
            let mut filtered_entries = included_entries
                .iter()
                .filter(|entry| entry["sys"]["id"].to_string() == link_id.to_string())
                .take(1);
            let linked_entry = filtered_entries.next();
            if let Some(entry) = linked_entry {
                let mut entry = entry.clone();
                self.resolve_entry(&mut entry, &includes)?;
                *value = entry;
                //value["fields"] = entry["fields"].clone();
                //*value = entry["fields"].clone();
            }
        } else if link_type == "Asset" {
            // TODO: Asset
        } else {
            unimplemented!();
        }

        //*value = value["fields"].clone();
        Ok(())
    }
}
