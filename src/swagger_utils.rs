use crate::models;
use crate::types;

pub async fn get_definition(request_url: &str) -> Result<types::Schemas, String> {
    let response = reqwest::get(request_url).await;
    match response {
        Err(value) => Err(value.to_string()),
        Ok(value) => {
            let deserialized = value.json::<models::ApiDoc>().await;
            match deserialized {
                Err(value) => Err(value.to_string()),
                Ok(value) => Ok(value.components.schemas)
            }
        }
    }
}
