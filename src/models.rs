use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Space {
    pub name: String,
    locales: Vec<Locale>,
    #[serde(rename = "sys")]
    system_properties: SystemProperties,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
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

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct SystemProperties {
    id: String,
    //#[serde(rename = "linkType")]
    //link_type: String,
    // TODO: More fields
}
