-- Your SQL goes here
create table links
(
    id         serial                                 not null
        constraint links_pk
            primary key,
    redirect   text                                   not null,
    path       text                                   not null,
);