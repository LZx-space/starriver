DROP TABLE public."blog";

create table public.blog
(
    id         uuid primary key                                   not null,
    title      varchar,
    body       text,
    state      smallint,
    blogger_id varchar,
    create_at  timestamp with time zone default CURRENT_TIMESTAMP not null,
    update_at  timestamp with time zone default CURRENT_TIMESTAMP
);

comment on column public.blog.title is '标题';
comment on column public.blog.body is '正文';
comment on column public.blog.state is '状态';
comment on column public.blog.blogger_id is '作者';

alter table public.blog
    owner to postgres;

create index idx_blog_title
    on public.blog (title);

