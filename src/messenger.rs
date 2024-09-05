use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct MessengerParticipant {
    pub name: String,
}

#[derive(Deserialize, Clone, Debug)]
pub struct MessengerReaction {
    pub reaction: String,
    pub actor: String,
}
#[derive(Deserialize, Debug)]
pub struct MessengerMessage {
    pub sender_name: String,
    pub content: Option<String>,
    pub reactions: Option<Vec<MessengerReaction>>,
    pub timestamp_ms: i64,
}

#[derive(Deserialize, Debug)]
pub struct MessengerExtractFile {
    pub participants: Vec<MessengerParticipant>,
    pub messages: Vec<MessengerMessage>,
}