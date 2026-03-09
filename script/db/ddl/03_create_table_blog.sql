create table public.blog
(
    id         uuid primary key         not null,
    title      varchar                  not null,
    body       text                     not null,
    state      smallint                 not null,
    author_id  varchar                  not null,
    create_at  timestamp with time zone not null default CURRENT_TIMESTAMP,
    update_at  timestamp with time zone not null default CURRENT_TIMESTAMP
);

-- Column comments

comment on table public.blog is '博客';
comment on column public.blog.title is '标题';
comment on column public.blog.body is '正文';
comment on column public.blog.state is '状态';
comment on column public.blog.author_id is '作者';

create index idx_blog_title
    on public.blog (title);
