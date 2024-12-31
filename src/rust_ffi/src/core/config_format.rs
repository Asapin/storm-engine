use std::path::Path;

pub enum ConfigFormat {
    Ini,
    Toml
}

pub struct FindConfigResult {
    config_format: ConfigFormat,
    path: Path
}
