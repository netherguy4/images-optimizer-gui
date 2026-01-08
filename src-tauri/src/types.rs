use serde::{Deserialize, Serialize};
use std::sync::Mutex;

pub struct AppState {
    pub is_processing: Mutex<bool>,
}

#[derive(Debug, Deserialize, Clone)]
pub struct OptimizeConfig {
    pub paths: Vec<String>,
    pub jpg_q: u8,
    pub png_min: u8,
    pub png_max: u8,
    pub webp: bool,
    pub avif: bool,
    pub replace: bool,
    pub output_dir: Option<String>,
}

#[derive(Clone, Serialize)]
pub struct ProgressPayload {
    pub total: u64,
    pub done: u64,
    pub current_file: String,
}

#[derive(Serialize)]
pub struct FinalResult {
    pub total_files: u64,
    pub total_size_saved: u64,
    pub duration_secs: f64,
}

#[derive(Default)]
pub struct FileStats {
    pub bytes_saved: u64,
}
