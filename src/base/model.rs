use time::PrimitiveDateTime;

/// Someone in the group that may have sent [Message] or put [Reaction] on some
#[derive(Debug, Clone)]
pub struct Participant {
    pub id: i32,
    pub name: String,
}


/// [Message] are sent by [Participant] and can have [Reaction]
#[derive(Debug, Clone)]
pub struct Message {
    pub id: i32,
    pub timestamp_ms: PrimitiveDateTime,
    pub participant_id: Option<i32>,
    pub content: Option<String>,
    pub import_filename: Option<String>,
}

/// [Reaction] are put by [Participant] named "actor" on [Message].
/// 
/// The actor may be [None] when the original [Participant] is not found.
#[derive(Debug, Clone)]
pub struct Reaction {
    pub id: i32,
    pub reaction: String,
    pub actor_id: Option<i32>,
    pub message_id: i32,
}

pub struct Conversation {
    pub participants: Vec<Participant>,
    pub messages: Vec<Message>,
    pub reactions: Vec<Reaction>,
}
pub trait ConversationMarker {}
impl ConversationMarker for Conversation {}