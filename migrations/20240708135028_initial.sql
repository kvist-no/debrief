-- Add migration script here
create table summaries (
    id serial primary key,
    content text not null,
    date_time timestamp not null unique
);