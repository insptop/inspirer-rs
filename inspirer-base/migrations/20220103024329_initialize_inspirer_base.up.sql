-- Add up migration script here
create table if not exists `users` (
    id bigint unsigned not null auto_increment primary key,
    username varchar(40) not null,
    password varchar(120) not null,
    email varchar(80) default null,
    nickname varchar(40) not null default '',
    user_type smallint unsigned not null default 1,
    created_at timestamp not null default current_timestamp,
    updated_at timestamp not null default current_timestamp on update current_timestamp,
    constraint username_unique unique (username),
    constraint email_unique unique (email)
);