use crate::database::schemas::Assistant;
use ollama_rs::Ollama;

struct Ai {
    client: Ollama,
    assistant: Assistant,
    history_id: Option<String>,
}

impl Ai {
    pub fn new(assistant: Assistant) -> Self {
        let client = Ollama::new_default_with_history(256);
        Self {
            client,
            assistant,
            history_id: None,
        }
    }

    pub async fn completion(&mut self, prompt: &str) -> String {
        todo!()
    }
}
