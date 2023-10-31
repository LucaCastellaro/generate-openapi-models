mod models;
mod swagger_utils;
mod io_utils;
mod path_utils;
mod loaders_utils;

#[tokio::main]
async fn main() {
    let output_path = io_utils::text("Destination path");
    if output_path.is_err() {
        io_utils::response(io_utils::Response::Error, &io_utils::Payload::Obj(output_path.err()));
        return;
    }
    let output_path = output_path.unwrap();

    let output_path = path_utils::is_path_valid(output_path.as_str());
    if output_path.is_err() {
        io_utils::response(io_utils::Response::Error, &io_utils::Payload::Obj(output_path.err()));
        return;
    }
    let output_path = output_path.unwrap();
    io_utils::response(io_utils::Response::Valid, &io_utils::Payload::Obj(output_path));

    let swagger_url = io_utils::text("Swagger definition url");
    if swagger_url.is_err(){
        io_utils::response(io_utils::Response::Error, &io_utils::Payload::Obj(swagger_url.err()));
        return;
    }
    let swagger_url = swagger_url.unwrap();
    io_utils::response(io_utils::Response::Valid, &io_utils::Payload::Str::<&String>(&swagger_url));

    let message = format!("Downloading definition from {swagger_url}");
    let loader = loaders_utils::get_spinner(message.as_str());

    let definition = swagger_utils::get_definition(swagger_url.as_str()).await;
    let message = format!("Downloaded definition from {swagger_url}");
    loader.finish_with_message(message);

    match definition {
        Err(value) => io_utils::response(io_utils::Response::Error, &io_utils::Payload::Obj(value)),
        Ok(value) => {
            io_utils::response(io_utils::Response::Valid, &io_utils::Payload::Str::<&String>(&"Swagger definition is valid".to_string()));
            
            let loader = loaders_utils::get_spinner("Creating models from schema");

            let mut warnings = vec![];

            for (object_name, schema) in value {
                let message = format!("Working on {object_name}");
                loader.set_message(message);
                
                if schema.properties.is_none(){
                    let message = format!("Skipped schema {object_name} because it has no properties");
                    warnings.push(message);
                    continue;
                }

                loader.set_message("Reading properties");

                let props: models::Properties = schema.properties.unwrap();

                loader.set_message("Creating output path");

                let output_path = output_path.to_path_buf();
                let output_path = output_path.to_str().unwrap();

                loader.set_message("Creating dotnet object");

                let object = swagger_utils::create_dotnet_object(&object_name, &props, output_path, &loader).await;
                let output = object.join("\n");

                let path = format!("{output_path}/{object_name}.cs");

                let message = format!("Writing file {path}");
                loader.set_message(message);

                let result = std::fs::write(&path, output);

                match result {
                    Ok(()) => {
                        let message = format!("File {path} created");
                        loader.finish_with_message(message)
                    },
                    Err(value) => {
                        io_utils::response(io_utils::Response::Error, &io_utils::Payload::Obj(value));
                    },
                }
            }
        
            loader.finish_with_message("Schema created");

            if !warnings.is_empty() {
                for warning in warnings {
                    io_utils::response(io_utils::Response::Warning, &io_utils::Payload::Str::<&String>(&warning));
                }
            }
        }
    }
}