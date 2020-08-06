use crate::http_client;
use crate::QueryBuilder;
use serde::{Deserialize, Serialize};
use serde_json::Value;

pub struct ContentfulClient {
    access_token: String,
    space_id: String,
    base_url: String,
}

impl ContentfulClient {
    pub fn new(access_token: String, space_id: String) -> ContentfulClient {
        ContentfulClient {
            base_url: "https://cdn.contentful.com/spaces".to_string(),
            access_token,
            space_id,
        }
    }

    pub async fn get_entry<T>(&self, entry_id: String) -> Result<T, Box<dyn std::error::Error>>
    where
        for<'a> T: Serialize + Deserialize<'a>,
    {
        let environment = "master";
        let url = format!(
            "{base_url}/{space_id}/environments/{environment}/entries/{entry_id}",
            base_url = &self.base_url,
            space_id = &self.space_id,
            environment = &environment,
            entry_id = &entry_id
        );
        let mut json = http_client::get::<Value>(&url, &self.access_token).await?;
        let entry_value = json.get_mut("fields").unwrap();
        dbg!(&entry_value);
        let entry_string = entry_value.to_string();
        let entry = serde_json::from_str::<T>(&entry_string.as_str())?;
        Ok(entry)
    }

    pub async fn get_entries<T>(
        &self,
        query_builder: Option<QueryBuilder>,
    ) -> Result<Vec<T>, Box<dyn std::error::Error>>
    where
        for<'a> T: Serialize + Deserialize<'a>,
    {
        let query_string = if let Some(query_builder) = query_builder {
            Some(query_builder.build())
        } else {
            None
        };

        self.get_entries_by_query_string::<T>(query_string).await
    }

    pub async fn get_entries_by_query_string<T>(
        &self,
        query_string: Option<String>,
    ) -> Result<Vec<T>, Box<dyn std::error::Error>>
    where
        for<'a> T: Serialize + Deserialize<'a>,
    {
        log::debug!("query_string: {:?}", &query_string);
        let environment = "master";
        let query_string = query_string.unwrap_or("".to_string());
        let url = format!(
            "{base_url}/{space_id}/environments/{environment}/entries{query_string}",
            base_url = &self.base_url,
            space_id = &self.space_id,
            environment = &environment,
            query_string = &query_string
        );
        let mut json = http_client::get::<Value>(&url, &self.access_token).await?;
        let includes = json.get("includes").unwrap().clone(); // TODO: Check if clone can be avoided
        let items = json.get_mut("items").unwrap();

        if items.is_array() {
            self.get_array(items, &includes)?;
            let ar_string = items.to_string();
            let entries = serde_json::from_str::<Vec<T>>(&ar_string.as_str())?;
            Ok(entries)
        } else {
            unimplemented!();
        }
    }

    pub async fn get_entries_by_type<T>(
        &self,
        content_type: String,
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

    fn get_array(
        &self,
        value: &mut Value,
        includes: &Value,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let items = value.as_array_mut().unwrap();
        for item in items {
            if item.is_object() {
                self.get_object(item, &includes)?;
            } else {
                unimplemented!();
            }
        }
        Ok(())
    }

    fn get_object(
        &self,
        value: &mut Value,
        includes: &Value,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let sys_type = value["sys"]["type"].clone();
        if sys_type == "Entry" {
            if let Some(fields) = value.get_mut("fields") {
                if fields.is_object() {
                    let entry_object = fields.as_object_mut().unwrap();
                    for (_field_name, field_value) in entry_object {
                        if field_value.is_object() {
                            self.get_object(field_value, &includes)?;
                        } else if field_value.is_array() {
                            self.get_array(field_value, &includes)?;
                        }
                    }
                    *value = fields.clone();
                } else {
                    unimplemented!();
                }
            } else {
                unimplemented!();
            }
        } else if sys_type == "Link" {
            self.resolve_link(value, &includes)?;
            *value = value["fields"].clone();
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

    fn resolve_link(
        &self,
        value: &mut Value,
        includes: &Value,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let link_type = value["sys"]["linkType"].clone();
        let link_id = value["sys"]["id"].clone();
        if link_type == "Entry" {
            let included_entries = includes["Entry"].as_array().unwrap();
            let mut filter_entries = included_entries
                .iter()
                .filter(|entry| entry["sys"]["id"].to_string() == link_id.to_string())
                .take(1);
            let included_entry = filter_entries.next();
            if let Some(entry) = included_entry {
                value["fields"] = entry["fields"].clone();
            }
        } else if link_type == "Asset" {
        } else {
            unimplemented!();
        }
        Ok(())
    }
}
