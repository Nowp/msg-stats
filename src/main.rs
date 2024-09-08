use crate::messenger::MessengerConversation;

use crate::base::adapter::{ConversationConverter, ConversationLoader, MergeImportFiles};
use crate::base::unnest::Unnest;
use itertools::Itertools;
use sqlx::postgres::PgPoolOptions;
use sqlx::Error;
use std::fs;

mod messenger;
mod base;

fn parse_file(path: String) -> serde_json::Result<MessengerConversation> {
    let content: String = fs::read_to_string(path).expect("Cannot read file");
    serde_json::from_str::<MessengerConversation>(&content)
}


#[tokio::main]
async fn main() -> Result<(), Error> {
    tokio::spawn(async {
        // Create a connection pool
        let pool = PgPoolOptions::new()
            .max_connections(10)
            .connect("postgres://postgres:messengerinput@localhost/ouaislesboys").await?;

        let paths: Vec<String> = fs::read_dir("./messages/fixed")?
            .into_iter()
            .filter_map(|x|
                match x {
                    Ok(dir) => {
                        if dir.metadata().unwrap().is_file() && dir.file_name().to_str().unwrap().contains(".json") { Some(dir.path()) } else { None }
                    }
                    Err(_) => { panic!("Panic") }
                }
            )
            .map(|path| path.to_str().unwrap().to_string())
            .collect();

        let mut conversation = paths.into_iter()
            .map(|path| parse_file(path).unwrap())
            .collect::<Vec<MessengerConversation>>()
            .into_iter()
            .merge_import_files()
            .map(MessengerConversation::convert)
            .unwrap();


        conversation.load_participants(&pool).await?;
        conversation.load_messages(&pool).await?;
        conversation.load_reactions(&pool).await?;

        Ok::<(), Error>(())
    }).await.expect("Et bah non").expect("SQL error");
    Ok(())
}
