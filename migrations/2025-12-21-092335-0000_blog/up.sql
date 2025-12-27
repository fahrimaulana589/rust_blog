-- Your SQL goes here
create table categories (
    id integer not null primary key,
    name varchar(255) not null,
    created_at timestamp not null default current_timestamp,
    updated_at timestamp not null default current_timestamp
);

create table tags (
    id integer not null primary key,
    name varchar(255) not null,
    created_at timestamp not null default current_timestamp,
    updated_at timestamp not null default current_timestamp
);

create table blog (
    id integer not null primary key,
    title varchar(255) not null,
    content text not null,
    category_id integer not null references categories(id),
    created_at timestamp not null default current_timestamp,
    updated_at timestamp not null default current_timestamp
);

create table blog_tags (
    blog_id integer not null references blog(id) on delete cascade,
    tag_id integer not null references tags(id) on delete restrict,
    primary key (blog_id, tag_id)
);