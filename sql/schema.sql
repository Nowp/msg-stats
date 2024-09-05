create table participants
(
    id   serial
        constraint participants_pk
            primary key,
    name text
);

create table messages
(
    content        text,
    participant_id integer
        constraint messages_participants_id_fk
            references participants,
    timestamp_ms   timestamp not null,
    id             serial
        constraint messages_pk
            primary key
);

create table reactions
(
    reaction   varchar not null,
    actor_id   integer
        constraint reactions_participants_id_fk
            references participants,
    message_id integer not null
        constraint reactions_messages_id_fk
            references messages,
    id         serial
        constraint reactions_pk
            primary key
);

