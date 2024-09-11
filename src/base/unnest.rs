use std::vec::IntoIter;
use time::PrimitiveDateTime;
use crate::base::model::{Message, Participant, Reaction};

pub trait Unnest {
    type Output;
    fn unnest(self) -> Self::Output;
}

impl Unnest for IntoIter<Participant> {
    type Output = (Vec<i32>, Vec<String>);

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
    type Output = (Vec<i32>, Vec<PrimitiveDateTime>, Vec<Option<String>>, Vec<Option<String>>, Vec<Option<i32>>,);

    fn unnest(self) -> Self::Output {
        let (
            mut ids,
            mut timestamps,
            mut import_filenames,
            mut contents,
            mut participant_ids
        ): Self::Output = (vec![], vec![], vec![], vec![], vec![]);

        self.for_each(|message| {
            ids.push(message.id);
            timestamps.push(message.timestamp_ms);
            participant_ids.push(message.participant_id);
            contents.push(message.content);
            import_filenames.push(message.import_filename);
        });

        (ids, timestamps, import_filenames, contents, participant_ids)
    }
}
impl Unnest for IntoIter<Reaction> {
    type Output = (Vec<i32>, Vec<String>, Vec<Option<i32>, >, Vec<i32>);

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

#[cfg(test)]
mod tests {
    use time::{Date, Month, Time};
    use super::*;


    #[test]
    fn unnest_participants_success() {
        let participants: Vec<Participant> = vec![
            Participant {
                id: 0,
                name: String::from("John Doe"),
            },
            Participant {
                id: 1,
                name: String::from("Steve John"),
            },
        ];

        let (ids, names) = participants.into_iter().unnest();

        assert_eq!(ids, vec![0, 1]);
        assert_eq!(names, vec![String::from("John Doe"), String::from("Steve John")]);
    }

    #[test]
    fn unnest_messages_success() {
        let messages: Vec<Message> = vec![
            Message {
                id: 0,
                timestamp_ms: PrimitiveDateTime::new(Date::from_calendar_date(2000, Month::February, 15).unwrap(), Time::from_hms(10, 10, 10).unwrap()),
                import_filename: Some("file.json".to_string()),
                content: Some("I'm a message".to_string()),
                participant_id: Some(0)
            },
            Message {
                id: 2,
                timestamp_ms: PrimitiveDateTime::new(Date::from_calendar_date(2001, Month::December, 1).unwrap(), Time::from_hms(5, 5, 5).unwrap()),
                import_filename: Some("file1.json".to_string()),
                content: Some("I'm a messaaaage".to_string()),
                participant_id: Some(3)
            },
        ];

        let message_copy0 = messages.first().unwrap().clone();
        let message_copy1 = messages.get(1).unwrap().clone();
        let (ids, timestamp, files, contents, participants) = messages.into_iter().unnest();

        assert_eq!(ids, vec![message_copy0.clone().id, message_copy1.clone().id]);
        assert_eq!(timestamp, vec![message_copy0.clone().timestamp_ms, message_copy1.clone().timestamp_ms]);
        assert_eq!(files, vec![message_copy0.import_filename.clone(), message_copy1.import_filename.clone()]);
        assert_eq!(contents, vec![message_copy0.content.clone(), message_copy1.content.clone()]);
        assert_eq!(participants, vec![message_copy0.participant_id, message_copy1.participant_id]);
    }

    #[test]
    fn unnest_reactions_success() {
        let messages: Vec<Reaction> = vec![
            Reaction {
                id: 0,
                reaction: String::from("😆"),
                actor_id: Some(0),
                message_id: 0
            },
            Reaction {
                id: 1,
                reaction: String::from("😄"),
                actor_id: Some(1),
                message_id: 1
            },
        ];

        let reaction_copy0 = messages.first().unwrap().clone();
        let reaction_copy1 = messages.get(1).unwrap().clone();
        let (ids, reactions, actors, messages) = messages.into_iter().unnest();

        assert_eq!(ids, vec![0, 1]);
        assert_eq!(reactions, vec![reaction_copy0.clone().reaction, reaction_copy1.clone().reaction]);
        assert_eq!(actors, vec![reaction_copy0.actor_id, reaction_copy1.actor_id]);
        assert_eq!(messages, vec![reaction_copy0.message_id, reaction_copy1.message_id]);
    }
}