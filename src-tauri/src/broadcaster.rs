use tokio::sync::broadcast;
use serde::{Serialize, Deserialize};

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct TranslationEvent {
    pub original_text: String,
    pub translated_text: String,
    pub source_lang: String,
    pub target_lang: String,
    pub region: String,
    pub captured_image: Option<String>, // Base64
    pub processed_image: Option<String>, // Base64
}

pub struct Broadcaster {
    pub tx: broadcast::Sender<TranslationEvent>,
}

impl Broadcaster {
    pub fn new() -> Self {
        let (tx, _) = broadcast::channel(100);
        Self { tx }
    }

    pub fn send(&self, event: TranslationEvent) -> Result<usize, broadcast::error::SendError<TranslationEvent>> {
        self.tx.send(event)
    }

    pub fn subscribe(&self) -> broadcast::Receiver<TranslationEvent> {
        self.tx.subscribe()
    }
}

impl Default for Broadcaster {
    fn default() -> Self {
        Self::new()
    }
}
