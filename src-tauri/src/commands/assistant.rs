#[tauri::command(async, rename_all = "snake_case")]
pub async fn completion() -> Result<String, String> {
    todo!()
}
