use crate::{models, struct_utils};

impl models::ApiDocProperty {
    pub async fn get_property_type(&self, property_name: &String, output_path: &str) -> String {
        match self.type_name.as_ref() {
            None => self.get_property_type_from_ref(),
            Some(type_name) => {
                let property_type = type_name.get_property_type();
                self.handle_property_type(property_type, property_name, output_path)
            }
        }
    }

    fn get_property_type_from_ref(&self) -> String {
        match self.type_ref.as_ref() {
            Some(type_ref) => match type_ref.0.split("/").last() {
                Some(type_ref) => type_ref.to_string(),
                None => "object".to_string()
            },
            None => "object".to_string()
        }
    }

    fn get_property_type_from_format(&self) -> Result<String, ()> {
        match self.type_format.as_ref() {
            None => Err(()),
            Some(type_format) => match type_format.0.as_str() {
                "date-time" => Ok("DateTime".to_string()),
                &_ => Ok("string".to_string())
            } 
        }
    }

    fn get_property_type_from_enum(&self, property_name: &String, output_path: &str) -> String {
        match self.enum_definition.as_ref() {
            None => "string".to_string(),
            Some(value) => value.get_property_type(property_name, output_path)
        }
    }

    fn handle_string_property_type(&self, property_type: String, property_name: &String, output_path: &str) -> String {
        let property_type_from_format = self.get_property_type_from_format();
        match property_type_from_format {
            Ok(property_type_from_format) => property_type_from_format,
            Err(_) => {
                match property_type.as_str() {
                    "string" => self.get_property_type_from_enum(property_name, output_path),
                    &_ => "object".to_string()
                }
            },
        }
    }

    fn handle_property_type (&self, property_type: Result<String, String>, property_name: &String, output_path: &str) -> String {
        match property_type {
            Ok(value) => value,
            Err(property_type) => {
                match property_type.as_str() {
                    "string" => self.handle_string_property_type(property_type, property_name, output_path),
                    "array" => self.handle_array_property_type(property_name, output_path),
                    &_ => "object".to_string()
                }
            },
        }
    }

    fn handle_array_property_type(&self, property_name: &String, output_path: &str) -> String {
        let property_type = match self.items.as_ref() {
            Some(items) => {
                let r#ref = items.get_ref();
                match r#ref {
                    Some(value) => {
                        let type_from_ref = value.0.as_str().split("/").last().unwrap();
                        Ok(format!("{type_from_ref}[]"))
                    },
                    None => {
                        let r#type = items.get_type_name();
                        match r#type {
                            Some(value) => match struct_utils::get_property_type_from_type_name(value.0) {
                                Ok(value) => Ok(value),
                                Err(_) => Err("object".to_string())
                            },
                            None => Err("object".to_string()),
                        }
                    },
                }
            },
            None => Err("object".to_string()),
        };
        match property_type {
            Ok(property_type) => property_type,
            Err(property_type) => {
                match property_type.as_str() {
                    "string" => self.handle_string_property_type(property_type, property_name, output_path),
                    &_ => "object".to_string()
                }
            },
        }
    }
}
