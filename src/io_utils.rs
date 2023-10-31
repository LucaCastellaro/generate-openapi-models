use std::{io::Error, fmt::Debug};
use dialoguer::Input;
use console::Style;
use lazy_static::lazy_static;

use crate::loaders_utils;

lazy_static! {
    static ref ERROR_RESPONSE: Style = Style::new().color256(124).bold();
    static ref WARNING_RESPONSE: Style = Style::new().color256(184).bold();
    static ref VALID_RESPONSE: Style = Style::new().color256(034).bold();
}

pub enum Response {
    Error,
    Warning,
    Valid
}

pub enum Payload<'a, T> {
    Str(&'a String),
    Obj(T)
}

pub fn text(label: &str) -> Result<String, Error> {
    return Input::<String>::new()
        .with_prompt(label)
        .interact_text();
}

pub fn response<T>(response: Response, payload: &Payload<T>) where T : Debug {
    match response {
        Response::Error => match payload {
            Payload::Str(value) =>  println!("{} {}", (*ERROR_RESPONSE).apply_to(loaders_utils::OK), value),
            Payload::Obj(value) =>  println!("{} {:#?}", (*ERROR_RESPONSE).apply_to(loaders_utils::OK), value),
        },
        Response::Warning => match payload {
            Payload::Str(value) =>  println!("{} {}", (*WARNING_RESPONSE).apply_to(loaders_utils::OK), value),
            Payload::Obj(value) =>  println!("{} {:#?}", (*WARNING_RESPONSE).apply_to(loaders_utils::OK), value),
        },
        Response::Valid => match payload {
            Payload::Str(value) =>  println!("{} {}", (*VALID_RESPONSE).apply_to(loaders_utils::OK), value),
            Payload::Obj(value) =>  println!("{} {:#?}", (*VALID_RESPONSE).apply_to(loaders_utils::OK), value),
        }
    };
}