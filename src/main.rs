use crate::messenger::{MessengerExtractFile, MessengerReaction};
use base::model::{Participant, Reaction, Record};

use futures::future::join_all;
use sqlx::postgres::PgPoolOptions;
use sqlx::{query, query_as, Error, Pool, Postgres};
use std::collections::HashMap;
use std::fs;
use time::{Duration, OffsetDateTime, PrimitiveDateTime};

mod messenger;
mod base;

async fn parse_file(path: String) -> Result<MessengerExtractFile, serde_json::Error> {
    let content: String = fs::read_to_string(path).expect("Cannot read file");
    serde_json::from_str::<MessengerExtractFile>(&content)
}

async fn insert_participants(pool: &Pool<Postgres>, extract_file: &MessengerExtractFile) -> Result<HashMap<Option<String>, Participant>, Error> {
    let names: Vec<String> = extract_file.participants.iter().map(|participant| participant.name.clone()).collect();
    
    let insert_participants = query!(
            "
            INSERT INTO participants(name)
            SELECT UNNEST($1::text[])
            ",
            &names[..]
        );

    insert_participants.execute(pool).await.expect("Cannot insert participants");

    
    let inserted_participants = query_as!(Participant, "SELECT id, name FROM participants").fetch_all(pool).await?;
    let participants_by_name: HashMap<Option<String>, Participant> = inserted_participants.into_iter().map(|x| (x.clone().name, x)).collect::<HashMap<_, _>>();

    Ok(participants_by_name)
}

async fn insert_messages(pool: &Pool<Postgres>, extract_file: MessengerExtractFile, participants: &HashMap<Option<String>, Participant>) -> Result<(), Error> {
    println!("Inserting {} messages", &extract_file.messages.len());

    let mut timestamp: Vec<PrimitiveDateTime> = vec![];
    let mut messenger_reactions: Vec<Vec<MessengerReaction>> = vec![];
    let mut content: Vec<Option<String>> = vec![];
    let mut participants_id: Vec<Option<i32>> = vec![];

    extract_file.messages.into_iter().for_each(|message| {
        let t = OffsetDateTime::from_unix_timestamp(0).unwrap() + Duration::milliseconds(message.timestamp_ms);
        let date = t.date();
        let time = t.time();

        timestamp.push(PrimitiveDateTime::new(date, time));
        messenger_reactions.push(message.reactions.unwrap_or_default());
        content.push(message.content.clone());
        participants_id.push(participants.get(&Some(message.sender_name.clone())).map(|x| x.id));
    });

    let insert_messages = query_as!(
        Record,
        "
        INSERT INTO messages(timestamp_ms, participant_id, content)
        SELECT * FROM UNNEST($1::timestamp[], $2::int[], $3::text[])
        RETURNING id
        ",
        &timestamp[..],
        &participants_id[..] as &[Option<i32>],
        &content[..]  as &[Option<String>]
    );

    let ids: Vec<i32> = insert_messages.fetch_all(pool).await?.into_iter().map(|x| x.id).collect();

    let reactions: Vec<Reaction> = messenger_reactions.into_iter()
        .enumerate()
        .flat_map(|(pos, reactions)| reactions.into_iter().map(move |x| (pos, x)))
        .map(|(pos, reaction)| Reaction {
            id: pos as i32,
            message_id: **&ids.get(pos).expect("Message id not found"),
            reaction: reaction.reaction,
            actor_id: participants.get(&Some(reaction.actor)).map(|x|x.id),
        }).collect();

    let messages_ids: Vec<i32> = reactions.iter().map(|x| x.message_id.clone()).collect();
    let reaction: Vec<String> = reactions.iter().map(|x| x.reaction.clone()).collect();
    let actor_ids: Vec<Option<i32>> = reactions.iter().map(|x| x.actor_id.clone()).collect();

    query!(
        "
        INSERT INTO reactions(reaction, actor_id, message_id)
        SELECT * FROM UNNEST($1::text[], $2::int[], $3::int[])
        ",
        &reaction[..],
        &actor_ids[..] as &[Option<i32>],
        &messages_ids[..]
    ).execute(pool).await?;

    Ok(())
}

#[tokio::main]
async fn main() -> Result<(), Error> {
    tokio::spawn(async {
        // Create a connection pool
        let pool = PgPoolOptions::new()
            .max_connections(1)
            .connect("postgres://postgres:paco@localhost/ouaislesboys").await?;

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

        let extract_file = parse_file(paths.get(0).unwrap().clone()).await.expect("Error parsing file");
        let participants_future = insert_participants(&pool, &extract_file);

        let participants = participants_future.await?;

        let parsing_futures: Vec<_> = paths.into_iter().map(|path| parse_file(path)).collect();
        let message_futures: Vec<_> = join_all(parsing_futures).await.into_iter()
            .map(|extract_file| extract_file.unwrap())
            .map(|messenger_extract_file: MessengerExtractFile| insert_messages(&pool, messenger_extract_file, &participants))
            .collect();

        // join_all(message_futures).await;
        
        for f in message_futures {
            f.await;
        }
        Ok::<(), Error>(())
    }).await.expect("Et bah non").expect("SQL error");
    Ok(())
}
