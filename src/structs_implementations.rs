use crate::{types, dotnet_utils, struct_utils};

impl types::PropertyTypeName {
    pub fn get_property_type(&self) -> Result<String, String> {
        let type_name = self.0.to_owned();
        struct_utils::get_property_type_from_type_name(type_name)
    }
}

impl types::PropertyEnum {
    pub fn get_property_type(&self, property_name: &String, output_path: &str) -> String {
        let keys = self.0.to_owned();
        dotnet_utils::create_enum(property_name, keys, output_path);
        property_name.to_string()
    }
}

impl types::PropertyArray {
    pub fn get_ref(&self) -> Option<types::PropertyTypeRef> {
        let type_ref = self.0.get("$ref");
        match type_ref {
            None => None,
            Some(type_ref) => {
                let type_ref = type_ref.to_owned() as types::Ref;
                return Some(types::PropertyTypeRef(type_ref));
            }
        }
    }
    pub fn get_type_name(&self) -> Option<types::PropertyTypeName> {
        let type_name = self.0.get("type");
        match type_name {
            None => None,
            Some(type_name) => {
                let type_name = type_name.to_owned() as types::Name;
                return Some(types::PropertyTypeName(type_name));
            }
        }
    }
}