use crate::base::model::{Message, Participant, Reaction};
use crate::base::unnest::Unnest;
use itertools::Itertools;
use sqlx::{query_as, Error, Pool, Postgres};
use std::slice::Iter;

pub(crate) async fn insert_participants(pool: &Pool<Postgres>, participants: Iter<'_, Participant>) -> Result<(), Error> {
    let (ids, names): (Vec<i32>, Vec<String>) = participants.cloned().collect_vec().into_iter().unnest();

    let insert_participants = query_as!(
        Participant,
        "
        INSERT INTO participants(id, name)
        SELECT * FROM UNNEST($1::int[], $2::text[])
        ",
        &ids[..],
        &names[..]
    );

    let _inserted_participants = insert_participants.execute(pool).await?;

    Ok(())
}

pub(crate) async fn insert_messages(pool: &Pool<Postgres>, messages: Iter<'_, Message>) -> Result<(), Error> {
    let (ids, timestamp, _, content, participants_id) = messages.cloned().collect_vec().into_iter().unnest();

    let insert_messages = query_as!(
        Message,
        "
        INSERT INTO messages(id, timestamp_ms, participant_id, content)
        SELECT * FROM UNNEST($1::int[], $2::timestamp[], $3::int[], $4::text[])
        ",
        &ids[..],
        &timestamp[..],
        &participants_id[..] as &[Option<i32>],
        &content[..]  as &[Option<String>]
    );

    let _inserted_messages = insert_messages.execute(pool).await?;

    Ok(())
}

pub(crate) async fn insert_reactions(pool: &Pool<Postgres>, reactions: Iter<'_, Reaction>) -> Result<(), Error> {
    let (ids, reactions, actor_ids, message_ids) = reactions.cloned().collect_vec().into_iter().unnest();

    let insert_reactions = query_as!(
        Reaction,
        "
        INSERT INTO reactions(id, reaction, actor_id, message_id)
        SELECT * FROM UNNEST($1::int[], $2::text[], $3::int[], $4::int[])
        ",
        &ids[..],
        &reactions[..],
        &actor_ids[..] as &[Option<i32>],
        &message_ids[..]  as &[i32]
    );

    let _reactions = insert_reactions.execute(pool).await?;

    Ok(())
}