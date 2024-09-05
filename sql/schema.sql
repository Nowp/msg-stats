create table participants
(
    id   serial
        constraint participants_pk
            primary key,
    name text
);

create table messages
(
    id              serial
        constraint messages_pk
            primary key,
    timestamp_ms    timestamp not null,
    participant_id  integer
        constraint messages_participants_id_fk
            references participants,
    content         text,
    import_filename varchar
);

create table reactions
(
    id         serial
        constraint reactions_pk
            primary key,
    actor_id   integer
        constraint reactions_participants_id_fk
            references participants,
    message_id integer not null
        constraint reactions_messages_id_fk
            references messages,
    reaction   varchar not null
);
