use std::path::PathBuf;

mod models;
mod models_implementations;
mod swagger_utils;
mod io_utils;
mod path_utils;
mod loaders_utils;
mod types;
mod dotnet_utils;
mod structs_implementations;
mod struct_utils;

#[tokio::main]
async fn main() {
    let output_path = get_output_path();
    match output_path {
        Err(err) => {
            io_utils::response(io_utils::Response::Error, &io_utils::Payload::Obj(err));
            return;
        }
        Ok(output_path) => {
            let swagger_url = get_swagger_url();
            handle_swagger_url(swagger_url, &output_path).await;
        }
    };
}

fn get_output_path() -> Result<PathBuf, String> {
    let output_path = io_utils::text("Destination path");
    match output_path {
        Ok(value) => {
            let output_path = path_utils::is_path_valid(value.as_str());
            match output_path {
                Ok(value) => {
                    io_utils::response(io_utils::Response::Valid, &io_utils::Payload::Obj(&value));
                    Ok(value)
                },
                Err(value) => {
                    io_utils::response(io_utils::Response::Error, &io_utils::Payload::Obj(&value));
                    Err(value)
                }
            }
        },
        Err(value) => {
            Err(value.to_string())
        },
    }
}

fn get_swagger_url() -> Result<String, String> {
    let swagger_url = io_utils::text("Swagger definition url");
    match swagger_url {
        Ok(value) => {
            io_utils::response(io_utils::Response::Valid, &io_utils::Payload::Str::<&String>(&value));
            Ok(value)
        },
        Err(value) => Err(value.to_string()),
    }
}

async fn handle_swagger_url(swagger_url: Result<String, String>, output_path: &PathBuf) {
    match swagger_url {
        Err(err) => io_utils::response(io_utils::Response::Error, &io_utils::Payload::Obj(err)),
        Ok(swagger_url) => {
            let definition = download_swagger_definition(swagger_url.as_str()).await;
            handle_swagger_definition(definition, output_path).await;
        }
    };
}

async fn download_swagger_definition(swagger_url: &str) -> Result<types::Schemas, String> {
    let message = format!("Downloading definition from {swagger_url}");
    let loader = loaders_utils::get_spinner(message.as_str());

    let definition = swagger_utils::get_definition(swagger_url).await;
    match definition {
        Ok(value) => {
            let message = format!("Downloaded definition from {swagger_url}");
            loader.finish_with_message(message);
            Ok(value)
        },
        Err(value) => Err(value)
    }
    
}

async fn handle_swagger_definition(definition: Result<types::Schemas, String>, output_path: &PathBuf) {
    match definition {
        Err(value) => io_utils::response(io_utils::Response::Error, &io_utils::Payload::Obj(value)),
        Ok(definition) => {
            io_utils::response(io_utils::Response::Valid, &io_utils::Payload::Str::<&String>(&"Swagger definition is valid".to_string()));
            
            create_models_from_schema(definition, &output_path).await;
        }
    };
}

async fn create_models_from_schema(definition: types::Schemas, output_path: &PathBuf) {
    let loader = loaders_utils::get_spinner("Creating models from schema");

    let mut warnings = vec![];

    for (object_name, schema) in definition {
        let message = format!("Working on {object_name}");
        loader.set_message(message);
        
        if schema.properties.is_none() {
            let message = format!("Skipped schema {object_name} because it has no properties");
            warnings.push(message);
            continue;
        }

        loader.set_message("Reading properties");

        let props: types::Properties = schema.properties.unwrap();

        loader.set_message("Creating output path");

        let output_path = output_path.to_str().unwrap();

        loader.set_message("Creating dotnet object");

        let object = dotnet_utils::create_record(&object_name, &props, output_path, &loader).await;
        let output = object.join("\n");

        let path = format!("{output_path}/{object_name}.cs");

        let message = format!("Writing file {path}");
        loader.set_message(message);

        let result = std::fs::write(&path, output);

        match result {
            Ok(()) => {
                let message = format!("File {path} created");
                loader.set_message(message)
            },
            Err(value) => {
                io_utils::response(io_utils::Response::Error, &io_utils::Payload::Obj(value));
            },
        }
    }

    loader.finish_with_message("Schema created");

    print_warnings(warnings);
}

fn print_warnings(warnings: Vec<String>) {
    if warnings.is_empty() {
        return;
    }

    for warning in warnings {
        io_utils::response(io_utils::Response::Warning, &io_utils::Payload::Str::<&String>(&warning));
    }
}