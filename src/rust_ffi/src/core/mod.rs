pub mod config_format;
pub mod engine_setting;
pub mod engine_version;
pub mod layers;
pub mod vma;

pub static DEBUG: bool = true;
pub static SAFE_MODE: bool = false;
pub static VALIDATE_COLLISION_DATA: bool = DEBUG || SAFE_MODE;
