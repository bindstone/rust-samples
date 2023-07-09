create table if not exists todo
(
    id          int     not null,
    owner       varchar not null,
    name        varchar not null,
    description varchar not null,
    status      varchar not null,
    created     timestamp with time zone,
    modified    timestamp with time zone,
    PRIMARY KEY (id)
);

CREATE SEQUENCE todo_seq
    INCREMENT 1
    START 1;