use std::collections::HashMap;

use crate::models;

pub type Schemas = HashMap<String, models::ApiDocSchema>;

pub type Properties = HashMap<String, models::ApiDocProperty>; 

pub type Ref = String;

pub type Name = String;

type Format = String;

type Enum = Vec<String>;

type Array = HashMap<String, String>;

#[derive(serde::Deserialize, Debug)] pub struct PropertyFormat(pub Format);

#[derive(serde::Deserialize, Debug)] pub struct PropertyTypeName(pub Name);

#[derive(serde::Deserialize, Debug)] pub struct PropertyTypeRef(pub Ref);

#[derive(serde::Deserialize, Debug)] pub struct PropertyEnum(pub Enum);

#[derive(serde::Deserialize, Debug)] pub struct PropertyArray(pub Array);