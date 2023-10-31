use std::path::Path;

pub fn is_path_valid(project_path: &str) -> Result<&Path, String> {
    let path = Path::new(project_path);
    if !Path::exists(path) {
        return Err(format!("Percorso non valido: {:#?}", path));
    }
    return Ok(path);
}