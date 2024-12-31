use std::path::PathBuf;

pub enum EngineSettingsPathType {
    Stash,
    Logs,
    SaveData,
    Screenshots,
    ScriptCache,
    Sentry
}

pub struct EngineSettings {
    stash_folder: PathBuf,
    logs_folder: Option<PathBuf>,
}
