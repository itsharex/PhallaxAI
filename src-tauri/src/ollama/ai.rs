use crate::database::schemas::{Assistant, Config};
use ollama_rs::{
    generation::chat::{request::ChatMessageRequest, ChatMessage},
    Ollama,
};
use uuid::Uuid;

pub struct Ai {
    client: Ollama,
    assistant: Assistant,
    config: Config,
    history_id: Option<String>,
}

impl Ai {
    pub fn new(assistant: Assistant, config: Config) -> anyhow::Result<Self> {
        let client = Ollama::new_default_with_history(256);
        if assistant.config_id != config.id {
            Err(anyhow::anyhow!("Assistant config_id must match Config id"))?;
        }
        Ok(Self {
            client,
            assistant,
            config,
            history_id: None,
        })
    }

    /// Get the chat history
    pub fn get_history(&mut self) -> anyhow::Result<Vec<ChatMessage>> {
        if let Some(history_id) = &self.history_id {
            let res = self.client.get_messages_history(history_id.to_string());
            let res = res.unwrap().to_vec();
            Ok(res)
        } else {
            Err(anyhow::anyhow!("History id not found"))?
        }
    }

    /// Send a chat message
    pub async fn completion(&mut self, model: &str, prompt: &str) -> anyhow::Result<String> {
        if self.history_id.is_none() {
            self.history_id = Some(Uuid::new_v4().to_string());
            self.client.set_system_response(
                self.history_id.to_owned().unwrap(),
                self.assistant.instructions.clone(),
            )
        }

        let res = self
            .client
            .send_chat_messages_with_history(
                ChatMessageRequest::new(
                    model.to_string(),
                    vec![ChatMessage::user(prompt.to_string())],
                ),
                self.history_id.to_owned().unwrap(),
            )
            .await?;

        Ok(res.message.unwrap().content)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_completion() {
        let config = Config::default();
        let assistant = Assistant::default();
        let mut ai = Ai::new(assistant, config).unwrap();
        let completion = ai.completion("llama3.1", "Hello.").await;
        assert!(completion.is_ok(), "{:?}", completion);
        assert!(ai.history_id.is_some(), "{:?}", ai.history_id);
    }

    #[tokio::test]
    async fn test_get_history() {
        let config = Config::default();
        let assistant = Assistant::default();
        let mut ai = Ai::new(assistant, config).unwrap();
        let _ = ai.completion("llama3.1", "Hello.").await;
        let history = ai.get_history();
        assert!(history.is_ok(), "{:?}", history);
    }
}
