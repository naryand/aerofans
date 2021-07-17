-- create_posts.sql
create table if not exists posts (
    id bigserial primary key,
    text text not null,
    created_at timestamp with time zone not null default (current_timestamp at time zone 'utc')
);

-- seed db with some test data for local dev
insert into posts
(text)
values
('test post 1'),
('test post 2'),
('test post 3');
