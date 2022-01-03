use std::path::PathBuf;

pub struct EnviromentContext {
    pub config_file: Option<PathBuf>,
    pub work_dir: PathBuf,
}