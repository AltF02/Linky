-- Your SQL goes here
create table linky.links
(
    id         bigserial                              not null
        constraint links_pk
            primary key,
    redirect   text                                   not null,
    path       text                                   not null,
    created_at timestamp with time zone default now() not null
);

create unique index links_path_uindex
    on linky.links (path);