use std::{fs, path::Path};

pub fn read(path: &str) -> Option<String> {
    Some(
        fs::read_to_string(Path::new(path))
            .ok()?
            .trim_matches('\n')
            .to_string(),
    )
}