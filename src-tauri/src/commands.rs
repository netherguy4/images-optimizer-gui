use tauri::{State, Window};
use crate::types::{AppState, FinalResult, OptimizeConfig};
use crate::optimizer::perform_optimization;

#[tauri::command]
pub async fn run_optimization(
    window: Window,
    config: OptimizeConfig,
    state: State<'_, AppState>,
) -> Result<FinalResult, String> {
    {
        let mut processing = state.is_processing.lock().map_err(|_| "Failed to lock state")?;
        if *processing {
            return Err("Optimization is already in progress.".to_string());
        }
        *processing = true;
    }

    let window_clone = window.clone();
    let result = tauri::async_runtime::spawn_blocking(move || {
        perform_optimization(&window_clone, config)
    })
    .await;

    {
        let mut processing = state.is_processing.lock().map_err(|_| "Failed to lock state")?;
        *processing = false;
    }

    match result {
        Ok(Ok(res)) => Ok(res),
        Ok(Err(e)) => Err(e),
        Err(_) => Err("Task panicked or failed internally.".to_string()),
    }
}
