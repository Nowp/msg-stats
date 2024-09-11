use std::future::Future;
use std::vec::IntoIter;
use futures::future::join_all;
use itertools::Itertools;
use serde::Deserialize;
use sqlx::{Error, PgPool};
use time::{Duration, OffsetDateTime, PrimitiveDateTime};
use crate::base::adapter::{ConversationConverter, ConversationLoader, ConversationMarker, MergeImportFiles};
use crate::base::model::{Conversation, Message, Participant, Reaction};
use crate::base::database::{insert_messages, insert_participants, insert_reactions};

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
pub struct MessengerConversation {
    pub participants: Vec<MessengerParticipant>,
    pub messages: Vec<MessengerMessage>,
}

impl ConversationMarker for MessengerConversation {}


impl ConversationLoader<PgPool> for Conversation {
    async fn load_participants(&self, destination: &PgPool) -> Result<(), Error> {
        insert_participants(destination, self.participants.iter()).await?;
        Ok(())
    }

    fn load_messages(&self, destination: &PgPool, chunk_size: Option<usize>) -> Vec<impl Future<Output=Result<(), Error>>> {
        self.messages
            .chunks(chunk_size.unwrap_or(u32::MAX as usize))
            .into_iter()
            .map(|chunk| chunk.iter().cloned().collect_vec())
            .map(|messages| async move { insert_messages(destination, messages.iter()).await })
            .collect_vec()
    }

    async fn load_reactions(&self, destination: &PgPool) -> Result<(), Error> {
        insert_reactions(destination, self.reactions.iter()).await?;
        Ok(())
    }
}


impl ConversationConverter for MessengerConversation {
    fn convert(self) -> Conversation {
        let participants: Vec<Participant> = self.participants.into_iter().enumerate().map(|(pos, p)| Participant {
            id: pos as i32,
            name: p.name,
        }).collect();

        let mut reactions: Vec<Reaction> = vec![];
        let mut messages: Vec<Message> = vec![];

        self.messages.into_iter()
            .enumerate()
            .for_each(|(pos, messenger_message)| {
                let sender = participants.iter().find(|x| x.name == messenger_message.sender_name);
                let t = OffsetDateTime::from_unix_timestamp(0).unwrap() + Duration::milliseconds(messenger_message.timestamp_ms);
                let date = t.date();
                let time = t.time();

                let message = Message {
                    id: pos as i32,
                    timestamp_ms: PrimitiveDateTime::new(date, time),
                    content: messenger_message.content.clone(),
                    participant_id: sender.map(|sender| sender.id),
                    import_filename: None,
                };
                let mut msg_reactions: Vec<Reaction> = messenger_message.reactions
                    .unwrap_or_default()
                    .into_iter()
                    .map(|reaction| {
                        let actor = participants.iter().find(|x| x.name == reaction.actor);
                        Reaction {
                            id: 0,
                            reaction: reaction.reaction.clone(),
                            actor_id: actor.map(|actor| actor.id),
                            message_id: pos as i32,
                        }
                    }).collect();

                reactions.append(&mut msg_reactions);
                messages.push(message);
            });

        reactions = reactions.into_iter().enumerate().map(|(pos, reaction)| Reaction {
            id: pos as i32,
            reaction: reaction.reaction,
            message_id: reaction.message_id,
            actor_id: reaction.actor_id,
        }).collect();

        Conversation {
            participants,
            messages,
            reactions,
        }
    }
}

impl MergeImportFiles<MessengerConversation> for IntoIter<MessengerConversation> {
    fn merge_import_files(self) -> Option<MessengerConversation> {
        self.reduce(|mut acc, mut file| {
            acc.messages.append(&mut file.messages);
            MessengerConversation {
                participants: file.participants,
                messages: acc.messages,
            }
        })
    }
}