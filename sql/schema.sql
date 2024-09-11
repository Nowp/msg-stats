create table public.participants
(
    id   serial
        constraint participants_pk
            primary key,
    name text not null
);

create table public.messages
(
    content         text,
    participant_id  integer
        constraint messages_participants_id_fk
            references public.participants,
    timestamp_ms    timestamp not null,
    id              serial
        constraint messages_pk
            primary key,
    import_filename varchar
);


create table public.reactions
(
    reaction   varchar not null,
    actor_id   integer
        constraint reactions_participants_id_fk
            references public.participants,
    message_id integer not null
        constraint reactions_messages_id_fk
            references public.messages,
    id         serial
        constraint reactions_pk
            primary key
);


