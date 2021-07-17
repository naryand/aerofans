-- create_replies.sql
create table if not exists replies (
    id bigserial primary key,
    post_id bigint not null,
    text text not null,
    created_at timestamp with time zone not null default (current_timestamp at time zone 'utc'),
    constraint post_fk
        foreign key (post_id)
        references posts(id)
        on delete cascade
);

-- seed db with some test data for local dev
insert into replies
(text, post_id)
values
('first', 1),
('second', 2),
('test comment 1', 2),
('test comment 2', 3);