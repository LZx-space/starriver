CREATE TABLE public.blog
(
    id         uuid primary key         NOT NULL,
    title      character varying(30)    NOT NULL,
    body       text                     NOT NULL,
    state      smallint                 NOT NULL,
    author_id  uuid                     NOT NULL,
    create_at  timestamp with time zone NOT NULL DEFAULT CURRENT_TIMESTAMP,
    update_at  timestamp with time zone
);
COMMENT ON TABLE public.blog IS '博客';

-- Column comments

COMMENT ON COLUMN public.blog.title IS '标题';
COMMENT ON COLUMN public.blog.body IS '正文';
COMMENT ON COLUMN public.blog.state IS '状态';
COMMENT ON COLUMN public.blog.author_id IS '作者';

create index idx_blog_title
    on public.blog (title);
