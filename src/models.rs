use chrono::{DateTime, Utc};
use getset::{Getters, Setters};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize, Getters, Setters)]
#[getset(get = "pub", set = "pub")]
pub struct Space {
    pub name: String,
    locales: Vec<Locale>,
    #[serde(rename = "sys")]
    system_properties: SystemProperties,
}

#[derive(Clone, Debug, Serialize, Deserialize, Getters, Setters)]
#[getset(get = "pub", set = "pub")]
#[serde(rename_all = "camelCase")]
pub struct Locale {
    name: String,
    code: String,
    fallback_code: Option<String>,
    default: bool,
    optional: bool,
    content_management_api: bool,
    content_delivery_api: bool,
}

#[derive(Clone, Debug, Serialize, Deserialize, Getters, Setters)]
#[getset(get = "pub", set = "pub")]
#[serde(rename_all = "camelCase")]
pub struct SystemProperties {
    id: String,
    version: Option<i32>,
    revision: Option<i32>,
    created_at: Option<DateTime<Utc>>,
    updated_at: Option<DateTime<Utc>>,
    // TODO: More fields
}

impl SystemProperties {
    pub fn new(id: String) -> SystemProperties {
        SystemProperties {
            id,
            version: None,
            revision: None,
            created_at: None,
            updated_at: None,
        }
    }

    pub fn with_version(id: String, version: i32) -> SystemProperties {
        SystemProperties {
            id,
            version: Some(version),
            revision: None,
            created_at: None,
            updated_at: None,
        }
    }
}

#[derive(Clone, Debug, Serialize, Deserialize, Getters, Setters)]
#[getset(get = "pub", set = "pub")]
#[serde(rename_all = "camelCase")]
pub struct Entry<T> {
    sys: SystemProperties,
    fields: T,
}

impl<T> Entry<T>
where
    T: Serialize,
{
    pub fn new(entry: T, sys: SystemProperties) -> Entry<T> {
        Entry { sys, fields: entry }
    }
}
