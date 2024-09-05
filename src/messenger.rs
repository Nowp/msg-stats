use crate::base::adapter::Adapter;
use crate::base::model::Participant;
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

impl Adapter<Participant> for MessengerParticipant {
    type Type = MessengerParticipant;

    fn to_database_model(self) -> Participant {
        Participant {
            id: 0,
            name: Some(self.name),
        }
    }

    fn from_database_model(model: Participant) -> MessengerParticipant {
        MessengerParticipant {
            name: model.name.unwrap_or_default()
        }
    }
}