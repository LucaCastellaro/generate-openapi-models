use crate::models;
use crate::types;

pub async fn get_definition(request_url: &str) -> Result<types::Schemas, types::DynError> {
    let response = reqwest::get(request_url).await?;
    let deserialized = response.json::<models::ApiDoc>().await?;
    Ok(deserialized.components.schemas)
}
