-- Your SQL goes here
create table users (
    id integer primary key,
    username text not null unique,
    email text not null unique,
    password text not null
);