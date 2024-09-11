use std::future::Future;
use crate::base::model::{Conversation, ConversationMarker, Message};
use sqlx::Error;

pub trait ConversationConverter {
    fn convert(self) -> Conversation;
}

pub trait ConversationLoader<D>
where
    Self: ConversationMarker,
    Self: Sized
{
    async fn load_participants(&self, destination: &D) -> Result<(), Error>;
    fn load_messages(&self, destination: &D, chunk_size: Option<usize>) -> Vec<impl Future<Output = Result<(), Error>>>;
    async fn load_reactions(&self, destination: &D) -> Result<(), Error>;
}

pub trait MergeImportFiles<T>
where
    T: ConversationMarker,
{
    fn merge_import_files(self) -> Option<T>;
}