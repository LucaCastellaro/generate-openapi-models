use std::{path::{Path, PathBuf}, fs};

use crate::types;

pub fn is_path_valid(project_path: &str) -> Result<PathBuf, types::DynError> {
    let path = Path::new(project_path);
    if !Path::exists(path) {
        fs::create_dir(path)?;
    }
    return Ok(path.to_path_buf());
}