use time::PrimitiveDateTime;

pub struct Record {
    pub id: i32,
}


#[derive(Debug, Clone)]
pub struct Participant {
    pub id: i32,
    pub name: Option<String>,
}


#[derive(Debug)]
pub struct Message {
    pub id: i32,
    pub timestamp_ms: PrimitiveDateTime,
    pub participant_id: i32,
    pub content: Option<String>,
    pub import_filename: Option<String>,
}


#[derive(Debug)]
pub struct Reaction {
    pub id: i32,
    pub reaction: String,
    pub actor_id: Option<i32>,
    pub message_id: i32,
}

