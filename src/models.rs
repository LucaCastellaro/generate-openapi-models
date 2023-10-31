use std::collections::HashMap;

#[derive(serde::Deserialize, Debug)]
pub struct ApiDocComponents {
    pub schemas: Schemas
}

pub type Schemas = HashMap<String, ApiDocSchema>;

#[derive(serde::Deserialize, Debug)]
pub struct ApiDocSchema {
    pub r#type: Option<String>,
    pub properties: Option<Properties>
}

pub type Properties = HashMap<String, ApiDocProperty>; 

#[derive(serde::Deserialize, Debug)]
pub struct ApiDocProperty {
    #[serde(rename(deserialize = "type"))] pub type_name: Option<String>,
    pub format: Option<String>,
    #[serde(rename(deserialize = "$ref"))] pub type_ref: Option<String>,
    pub items: Option<HashMap<String, String>>,
    pub r#enum: Option<Vec<String>>
}

#[derive(serde::Deserialize, Debug)]
pub struct ApiDoc {
    pub components: ApiDocComponents
}