use std::vec::IntoIter;
use time::PrimitiveDateTime;
use crate::base::model::{Message, Participant, Reaction};

trait Unnest {
    type Output;
    fn unnest(self) -> Self::Output;
}

impl Unnest for IntoIter<Participant> {
    type Output = (Vec<i32>, Vec<Option<String>>);

    fn unnest(self) -> Self::Output {
        let (mut ids, mut names): Self::Output = (vec![], vec![]);
        self.for_each(|participant| {
            ids.push(participant.id);
            names.push(participant.name);
        });

        (ids, names)
    }
}

impl Unnest for IntoIter<Message> {
    type Output = (Vec<i32>, Vec<PrimitiveDateTime>, Vec<i32>, Vec<Option<String>>, Vec<Option<String>>);

    fn unnest(self) -> Self::Output {
        let (
            mut ids,
            mut timestamps,
            mut participant_ids,
            mut contents,
            mut import_filenames
        ): Self::Output = (vec![], vec![], vec![], vec![], vec![]);

        self.for_each(|message| {
            ids.push(message.id);
            timestamps.push(message.timestamp_ms);
            participant_ids.push(message.participant_id);
            contents.push(message.content);
            import_filenames.push(message.import_filename);
        });

        (ids, timestamps, participant_ids, contents, import_filenames)
    }
}

impl Unnest for IntoIter<Reaction> {
    type Output = (Vec<i32>, Vec<String>, Vec<Option<i32>,>, Vec<i32>);

    fn unnest(self) -> Self::Output {
        let (
            mut ids,
            mut reactions,
            mut actor_ids,
            mut message_ids
        ): Self::Output = (vec![], vec![], vec![], vec![]);

        self.for_each(|message| {
            ids.push(message.id);
            reactions.push(message.reaction);
            actor_ids.push(message.actor_id);
            message_ids.push(message.message_id);
        });

        (ids, reactions, actor_ids, message_ids)
    }
}