create extension if not exists "uuid-ossp";

create table users (
    id uuid default uuid_generate_v4() primary key,
    username varchar not null unique,
    email varchar not null unique,
    password_hash varchar not null,
    created_at timestamp not null default current_timestamp,
    updated_at timestamp not null default current_timestamp,
);