use std::fs;
use std::path::{Path, PathBuf};
use std::sync::atomic::{AtomicU64, Ordering};
use std::sync::Arc;
use std::time::Instant;

use rayon::prelude::*;
use tauri::{Emitter, Window};
use walkdir::WalkDir;

use crate::image_ops::{generate_avif, generate_webp, process_jpg, process_png};
use crate::tools::{get_png_tools, ToolPath};
use crate::types::{FileStats, FinalResult, OptimizeConfig, ProgressPayload};

pub fn perform_optimization(window: &Window, config: OptimizeConfig) -> Result<FinalResult, String> {
    let start_time = Instant::now();

    let (_tmp_dir, pq, oxi) = get_png_tools().map_err(|e| format!("Failed to setup tools: {}", e))?;
    let _ = window.emit("status_update", "Scanning files...");

    let file_tasks = collect_file_tasks(&config)?;
    let total_files = file_tasks.len() as u64;

    let _ = window.emit("progress", ProgressPayload {
        total: total_files,
        done: 0,
        current_file: "Starting...".into(),
    });

    let done_counter = Arc::new(AtomicU64::new(0));

    let total_saved = file_tasks.par_iter()
        .map(|(src, dest)| {
            let stats = process_single_file(
                src,
                dest,
                &config,
                &pq,
                &oxi,
                window,
                &done_counter,
                total_files
            );
            stats.bytes_saved
        })
        .sum();

    Ok(FinalResult {
        total_files,
        total_size_saved: total_saved,
        duration_secs: start_time.elapsed().as_secs_f64(),
    })
}

fn collect_file_tasks(config: &OptimizeConfig) -> Result<Vec<(PathBuf, PathBuf)>, String> {
    let mut tasks = Vec::new();
    let supported_exts = ["png", "jpg", "jpeg"];

    let is_supported = |p: &Path| {
        p.extension()
            .map(|ext| supported_exts.contains(&ext.to_string_lossy().to_lowercase().as_str()))
            .unwrap_or(false)
    };

    for p_str in &config.paths {
        let root_path = Path::new(p_str);
        if !root_path.exists() { continue; }

        if root_path.is_dir() {
            for entry in WalkDir::new(root_path).into_iter().filter_map(|e| e.ok()) {
                if is_supported(entry.path()) {
                    let dest = resolve_output_path(entry.path(), root_path, config);
                    tasks.push((entry.path().to_path_buf(), dest));
                }
            }
        } else if is_supported(root_path) {
            let dest = resolve_output_path(root_path, root_path, config);
            tasks.push((root_path.to_path_buf(), dest));
        }
    }

    if tasks.is_empty() {
        return Err("No supported files found.".to_string());
    }

    Ok(tasks)
}

fn resolve_output_path(src: &Path, root_source: &Path, config: &OptimizeConfig) -> PathBuf {
    if let Some(ref out_dir_str) = config.output_dir {
        let out_base = Path::new(out_dir_str);
        if root_source.is_dir() {
            let relative = src.strip_prefix(root_source).unwrap_or(src);
            let dir_name = root_source.file_name().unwrap_or_default();
            out_base.join(dir_name).join(relative)
        } else {
            out_base.join(src.file_name().unwrap_or_default())
        }
    } else if config.replace {
        src.to_path_buf()
    } else {
        let stem = src.file_stem().unwrap_or_default().to_string_lossy();
        let ext = src.extension().unwrap_or_default().to_string_lossy();
        let new_name = format!("{}__optimized.{}", stem, ext);
        src.parent().unwrap_or(Path::new(".")).join(new_name)
    }
}

#[allow(clippy::too_many_arguments)]
fn process_single_file(
    src: &Path,
    dest: &Path,
    config: &OptimizeConfig,
    pq: &ToolPath,
    oxi: &ToolPath,
    window: &Window,
    done_counter: &Arc<AtomicU64>,
    total_files: u64
) -> FileStats {
    let _ = window.emit("file_start", src.file_name().unwrap_or_default().to_string_lossy());

    if src != dest {
        if let Some(parent) = dest.parent() {
            let _ = fs::create_dir_all(parent);
        }
        if fs::copy(src, dest).is_err() {
            return FileStats::default();
        }
    }

    let original_size = fs::metadata(src).map(|m| m.len()).unwrap_or(0);
    let mut bytes_saved = 0;

    if config.webp || config.avif {
        if let Ok(img) = image::open(src) {
            if config.webp {
                let _ = generate_webp(&img, dest, 75.0, original_size);
            }
            if config.avif {
                let _ = generate_avif(&img, dest, original_size);
            }
        }
    }

    let ext = dest.extension().unwrap_or_default().to_string_lossy().to_lowercase();

    if ext == "png" {
        bytes_saved = process_png(dest, pq, oxi, config.png_min, config.png_max);
    } else if ["jpg", "jpeg"].contains(&ext.as_str()) {
        bytes_saved = process_jpg(dest, config.jpg_q);
    }

    let done = done_counter.fetch_add(1, Ordering::Relaxed) + 1;
    let _ = window.emit("progress", ProgressPayload {
        total: total_files,
        done,
        current_file: src.file_name().unwrap_or_default().to_string_lossy().to_string(),
    });

    FileStats { bytes_saved }
}
