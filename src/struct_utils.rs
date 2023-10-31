use crate::types;

pub fn get_property_type_from_type_name(type_name: types::Name) -> Result<String, String> {
    match type_name.as_str() {
        "integer" => Ok("int".to_string()),
        "number" => Ok("decimal".to_string()),
        "boolean" => Ok("bool".to_string()),
        "string" => Err("string".to_string()),
        "array" => Err("array".to_string()),
        &_ => Err("object".to_string())
    }
}