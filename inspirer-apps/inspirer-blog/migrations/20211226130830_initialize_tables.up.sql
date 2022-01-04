create table if not exists `inspirer_blog_users` (
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

create table if not exists `inspirer_blog_contents` (
    id bigint unsigned not null auto_increment primary key,
    name varchar(160) default null,
    title varchar(140) not null,
    keywords varchar(200) not null default '',
    description varchar(500) not null default '',
    user_id bigint unsigned not null,
    status smallint unsigned not null default 0,
    published_at timestamp null,
    created_at timestamp not null default current_timestamp,
    updated_at timestamp not null default current_timestamp on update current_timestamp
);

create table if not exists `inspirer_blog_content_body` (
    id bigint unsigned not null primary key,
    content mediumtext
);

