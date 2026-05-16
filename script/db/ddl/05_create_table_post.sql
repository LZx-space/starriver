CREATE TABLE public.post
(
    id           uuid primary key         NOT NULL,
    title        character varying(30)    NOT NULL,
    content      text                     NOT NULL,
    state        smallint                 NOT NULL,
    author_id    uuid                     NOT NULL,
    category_id  uuid                     NOT NULL,
    published_at timestamp with time zone,
    created_at   timestamp with time zone NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at   timestamp with time zone
);
COMMENT ON TABLE public.post IS '博文';

-- Column comments

COMMENT ON COLUMN public.post.title IS '标题';
COMMENT ON COLUMN public.post.content IS '正文';
COMMENT ON COLUMN public.post.state IS '状态';
COMMENT ON COLUMN public.post.author_id IS '作者';
COMMENT ON COLUMN public.post.category_id IS '分类';
COMMENT ON COLUMN public.post.published_at IS '发布时间';

create index idx_post_title
    on public.post (title);

create index idx_post_category_id
    on public.post (category_id);

create index idx_post_published_at
    on public.post (published_at);
