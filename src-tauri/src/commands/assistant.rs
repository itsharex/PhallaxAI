use crate::{
    database::schemas::{Assistant, Config},
    ollama::ai::Ai,
};

#[tauri::command(async, rename_all = "snake_case")]
pub async fn completion(
    model: String,
    prompt: String,
    assistant: Assistant,
    config: Config,
) -> Result<String, String> {
    let mut ai = Ai::new(assistant, config).map_err(|err| err.to_string())?;
    let res = ai.completion(&model, &prompt).await;
    match res {
        Ok(res) => Ok(res),
        Err(err) => Err(err.to_string()),
    }
}
