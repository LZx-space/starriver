create table public."user"
(
    id        uuid primary key         not null,
    username  character varying(100)   not null,
    password  character varying(100)   not null,
    create_at timestamp with time zone not null default CURRENT_TIMESTAMP,
    update_at timestamp with time zone          default CURRENT_TIMESTAMP,
    CONSTRAINT user_unique UNIQUE (username)
);
