create table public.article
(
    id        uuid                                               not null
        primary key,
    title     varchar,
    body      text,
    state     smallint,
    author_id varchar,
    create_at timestamp with time zone default CURRENT_TIMESTAMP not null,
    update_at timestamp with time zone default CURRENT_TIMESTAMP
);

comment on column public.article.title is '标题';
comment on column public.article.body is '正文';
comment on column public.article.state is '状态';
comment on column public.article.author_id is '作者';

alter table public.article
    owner to postgres;

create index idx_article_title
    on public.article (title);

