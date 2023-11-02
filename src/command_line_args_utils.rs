use std::env;

use crate::constants;

pub fn get_command_line_args() -> Vec<String> {
    return env::args().collect();
}

pub fn print_help_menu() {
    println!("\t{}, {}\t\tSet the path in which the files will be created", constants::PATH_FAST, constants::PATH_FULL);
    println!("\t\t eg {} /Users/Temp/my_folder", constants::PATH_FAST);
    println!("\n\t{}, {}\tSet the url of the OpenApi definition", constants::URL_FAST, constants::URL_FULL);
    println!("\t\t eg {} https://localhost:5001/swagger/v1/api-docs", constants::URL_FAST);
}

pub fn user_asked_help(args: &Vec<String>) -> bool {
    args.contains(&constants::HELP.to_string())
}

pub fn get_destination_path(args: &Vec<String>) -> Option<String> {
    let arg = get_arg(args, &constants::PATH_FAST.to_string());
    match arg {
        Some(arg) => Some(arg),
        None => {
            let arg = get_arg(args, &constants::PATH_FULL.to_string());
            match arg {
                Some(arg) => Some(arg),
                None => None
            }
        }
    }
}

pub fn get_open_api_url(args: &Vec<String>) -> Option<String> {
    let arg = get_arg(args, &constants::URL_FAST.to_string());
    match arg {
        Some(arg) => Some(arg),
        None => {
            let arg = get_arg(args, &constants::URL_FULL.to_string());
            match arg {
                Some(arg) => Some(arg),
                None => None
            }
        }
    }
}

fn get_arg(args: &Vec<String>, arg_name: &String) -> Option<String> {
    let position = args.iter().position(|x| x == arg_name);

    match position {
        None => None,
        Some(position) => {
            let path = args[position + 1].to_owned();
            Some(path)
        }
    }
}