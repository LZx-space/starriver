CREATE TABLE public.article
(
    id         uuid primary key         NOT NULL,
    title      character varying(30)    NOT NULL,
    content    text                     NOT NULL,
    state      smallint                 NOT NULL,
    author_id  uuid                     NOT NULL,
    create_at  timestamp with time zone NOT NULL DEFAULT CURRENT_TIMESTAMP,
    update_at  timestamp with time zone
);
COMMENT ON TABLE public.article IS '文章';

-- Column comments

COMMENT ON COLUMN public.article.title IS '标题';
COMMENT ON COLUMN public.article.content IS '正文';
COMMENT ON COLUMN public.article.state IS '状态';
COMMENT ON COLUMN public.article.author_id IS '作者';

create index idx_article_title
    on public.article (title);
