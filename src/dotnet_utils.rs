use std::{path::Path, fs};

use convert_case::{Casing, Case};
use indicatif::ProgressBar;

use crate::types;

pub async fn create_record(object_name: &String, properties: &types::Properties, output_path: &str, loader: &ProgressBar) -> Vec<String> {
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

        let property_type = property.get_property_type(&camel_case_property_name, output_path).await;

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

pub fn create_enum(name: &String, keys: Vec<String>, output_path: &str) {
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
        lines.push(format!("\t{key},"));
    }

    // close enum
    lines.push("}".to_string());

    _ = fs::write(enum_path, lines.join("\n"));
}