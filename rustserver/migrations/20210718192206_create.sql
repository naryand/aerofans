-- create_users.sql
create table if not exists users (
    id bigserial primary key,
    username text not null unique,
    password char(60) not null,
    created_at timestamp with time zone not null default (current_timestamp at time zone 'utc')
);

-- seed db with some test data for local dev
insert into users
(username, password)
values
('testuser', '0');

-- create_tokens.sql
create table if not exists tokens (
    id text primary key,
    user_id bigint not null,
    expires_at timestamp with time zone not null,
    constraint user_fk
        foreign key (user_id)
        references users(id)
        on delete cascade
);

-- seed db with some test data for local dev
insert into tokens
(id, user_id, expires_at)
values
('LET_ME_IN', 1, (CURRENT_TIMESTAMP + INTERVAL '15 minutes') AT TIME ZONE 'utc');


-- create_posts.sql
create table if not exists posts (
    id bigserial primary key,
    author bigint not null,
    text text not null,
    created_at timestamp with time zone not null default (current_timestamp at time zone 'utc'),
    constraint author_fk
        foreign key (author)
        references users(id)
        on delete cascade
);

-- seed db with some test data for local dev
insert into posts
(text, author)
values
('test post 1', 1),
('test post 2', 1),
('test post 3', 1);

-- create_replies.sql
create table if not exists replies (
    id bigserial primary key,
    post_id bigint not null,
    author bigint not null,
    text text not null,
    created_at timestamp with time zone not null default (current_timestamp at time zone 'utc'),
    constraint post_fk
        foreign key (post_id)
        references posts(id)
        on delete cascade,
    constraint author_fk
        foreign key (author)
        references users(id)
        on delete cascade
);

-- seed db with some test data for local dev
insert into replies
(text, post_id, author)
values
('first', 1, 1),
('second', 2, 1),
('test comment 1', 2, 1),
('test comment 2', 3, 1);