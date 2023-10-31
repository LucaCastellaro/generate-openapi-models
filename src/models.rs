use crate::types;

#[derive(serde::Deserialize, Debug)]
pub struct ApiDocComponents {
    pub schemas: types::Schemas
}

#[derive(serde::Deserialize, Debug)]
pub struct ApiDocSchema {
    #[serde(rename(deserialize = "type"))] pub type_name: Option<String>,
    pub properties: Option<types::Properties>
}

#[derive(serde::Deserialize, Debug)]
pub struct ApiDocProperty {
    #[serde(rename(deserialize = "type"))] pub type_name: Option<types::PropertyTypeName>,
    #[serde(rename(deserialize = "$ref"))] pub type_ref: Option<types::PropertyTypeRef>,
    #[serde(rename(deserialize = "enum"))] pub enum_definition: Option<types::PropertyEnum>,
    #[serde(rename(deserialize = "format"))] pub type_format: Option<types::PropertyFormat>,

    pub items: Option<types::PropertyArray>
}

#[derive(serde::Deserialize, Debug)]
pub struct ApiDoc {
    pub components: ApiDocComponents
}