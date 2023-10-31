use std::{path::Path, fs};

use convert_case::{Case, Casing};
use indicatif::ProgressBar;

use crate::models;

pub async fn get_definition(request_url: &str) -> Result<models::Schemas, String> {
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

pub async fn create_dotnet_object(object_name: &String, properties: &models::Properties, output_path: &str, loader: &ProgressBar) -> Vec<String> {
    let mut output = vec![];

    // usings
    loader.set_message("Writing usings");
    output.push("using System.Text.Json.Serialization;".to_string());
    output.push("using OpenApi.Models.AutoGenerated;".to_string());
    output.push("using OpenApi.Models.AutoGenerated.Enums;".to_string());

    // namespace 
    loader.set_message("Writing namespace");
    output.push("\nnamespace OpenApi.Models.AutoGenerated;".to_string());

    // open record
    loader.set_message("Opening record");
    output.push(format!("\npublic sealed record {} {{", object_name));

    // properties
    loader.set_message("Writing properties");
    for (property_name, property) in properties {
        let camel_case_property_name = property_name.to_case(Case::UpperCamel);

        let message = format!("Adding property {camel_case_property_name} - json name = {property_name}");
        loader.set_message(message);

        let property_type = get_property_type(&camel_case_property_name, &property, output_path).await;

        let message = format!("Added property {property_type} {camel_case_property_name} - json name = {property_name}");
        loader.finish_with_message(message);

        let prop = format!("\t[JsonPropertyName(\"{property_name}\")] public {property_type}? {camel_case_property_name} {{ get; init; }}");
        output.push(prop);
    }

    // close class
    loader.set_message("Closing record");
    output.push("}".to_string());

    let message = format!("Create record {object_name}");
    loader.finish_with_message(message);

    output   
}

async fn get_property_type(property_name: &String, property: &models::ApiDocProperty, output_path: &str) -> String {
    let type_name = property.type_name.to_owned();
    match type_name {
        None => get_type_from_ref(&property.type_ref).await,
        Some(value) => get_type_from_item(property_name, value, property, output_path)
    }
}

async fn get_type_from_ref(r#ref: &Option<String>) -> String {
    match r#ref {
        None => "object".to_string(),
        Some(value) => value.split("/").last().unwrap().to_string()
    }
}

fn get_type_from_item(property_name: &String, type_name: String, property: &models::ApiDocProperty, output_path: &str) -> String {
    match type_name.as_str() {
        "integer" => "int".to_string(),
        "number" => "decimal".to_string(),
        "boolean" => "bool".to_string(),
        "string" => get_type_from_string_format(property_name, &property, output_path),
        "array" => get_array_type(property_name, type_name, property, output_path),
        &_ => "object".to_string()
    }
}

fn get_type_from_string_format(property_name: &String, property: &models::ApiDocProperty, output_path: &str) -> String {
    let format = property.format.to_owned();
    match format {
        None => get_type_from_enum(property_name, property, output_path),
        Some(value) => match value.as_str() {
            "date-time" => "DateTime".to_string(),
            _ => "string".to_string()
        } 
    }
}

fn get_type_from_enum(property_name: &String, property: &models::ApiDocProperty, output_path: &str) -> String {
    let r#enum = property.r#enum.to_owned();
    match r#enum {
        None => "string".to_string(),
        Some(value) => { 
            create_enum(property_name, value, output_path);
            property_name.to_string()
        }
    }
}

fn create_enum(name: &String, keys: Vec<String>, output_path: &str) {
    let enum_path = format!("{}/enums/{}.cs", output_path, name);

    if Path::new(&enum_path).exists() {
        return;
    }

    let enum_folder = format!("{}/Enums", output_path);
    let enum_folder = Path::new(&enum_folder);
    if !enum_folder.exists() {
        _ = fs::create_dir(enum_folder);
    }

    let mut lines = vec![];

    // usings
    lines.push("using System.Text.Json.Serialization;".to_string());
    lines.push("using OpenApi.Models.AutoGenerated;".to_string());
    lines.push("using OpenApi.Models.AutoGenerated.Enums;".to_string());

    // namespace 
    lines.push("\nnamespace OpenApi.Models.AutoGenerated.Enums;".to_string());

    // open enum
    lines.push(format!("\npublic enum {} {{", name));

    // enum keys
    for key in keys {
        lines.push(format!("\t{key}"));
    }

    // close enum
    lines.push("}".to_string());

    _ = fs::write(enum_path, lines.join("\n"));
}

fn get_array_type(property_name: &String, type_name: String, property: &models::ApiDocProperty, output_path: &str) -> String {
    let property_items = property.items.to_owned();

    match property_items {
        Some(items) => {
            let r#ref = items.get("$ref");
            if r#ref.is_some() {
                let type_from_ref = r#ref.unwrap().split("/").last().unwrap();
                return format!("{type_from_ref}[]");
            }

            let r#type = items.get("type");
            if r#type.is_some() {
                let type_from_item = match type_name.as_str() {
                    "integer" => "int".to_string(),
                    "number" => "decimal".to_string(),
                    "boolean" => "bool".to_string(),
                    "string" => get_type_from_string_format(property_name, &property, output_path),
                    &_ => "object".to_string()
                };
                return format!("{type_from_item}[]");
            }

            "object[]".to_string()
        },
        None => "object[]".to_string(),
    }
}