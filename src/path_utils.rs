use std::path::{Path, PathBuf};

pub fn is_path_valid(project_path: &str) -> Result<PathBuf, String> {
    let path = Path::new(project_path);
    if !Path::exists(path) {
        return Err(format!("Percorso non valido: {:#?}", path));
    }
    return Ok(path.to_path_buf());
}