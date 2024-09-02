use crate::{database::schemas::History, HISTORY_CACHE};
use ollama_rs::generation::chat::ChatMessage;
use serde_json;
use std::fs::File;
use std::io::{BufReader, BufWriter, Write};

/// Save the chat history to a file
pub async fn save_history(history: History, messages: Vec<ChatMessage>) -> anyhow::Result<()> {
    let history_cache = HISTORY_CACHE.lock().await;
    let file = File::create(history_cache.join(history.file_id.clone()))?;
    let mut writer = BufWriter::new(file);
    serde_json::to_writer(&mut writer, &messages)?;
    writer.flush()?;
    Ok(())
}

/// Load the chat history from a file
pub async fn load_history(history: History) -> anyhow::Result<Vec<ChatMessage>> {
    let history_cache = HISTORY_CACHE.lock().await;
    let file = File::open(history_cache.join(history.file_id.clone()))?;
    let reader = BufReader::new(file);
    let res = serde_json::from_reader(reader)?;
    Ok(res)
}

#[cfg(test)]
mod tests {
    use super::*;
    use tokio::fs;

    #[tokio::test]
    async fn test_save_load() {
        let history = History {
            id: 0,
            file_id: "test".to_string(),
            name: "test".to_string(),
            assistant_id: 0,
            created_at: chrono::Utc::now(),
            updated_at: chrono::Utc::now(),
        };
        let messages = vec![
            ChatMessage::system("You are a helpful assistant.".to_string()),
            ChatMessage::user("Hello".to_string()),
            ChatMessage::assistant("Hi".to_string()),
        ];
        save_history(history.clone(), messages.clone())
            .await
            .unwrap();
        let res = load_history(history.clone()).await.unwrap();
        fs::remove_file(HISTORY_CACHE.lock().await.join(history.file_id.clone()))
            .await
            .expect("Failed to remove file");
        let res = res
            .iter()
            .map(|m| m.content.clone())
            .collect::<Vec<String>>();
        let messages = messages
            .iter()
            .map(|m| m.content.clone())
            .collect::<Vec<String>>();
        assert_eq!(res, messages);
    }
}
