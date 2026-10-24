CREATE TABLE public.post
(
    id            uuid PRIMARY KEY         NOT NULL,
    title         character varying(50)    NOT NULL,
    content_html  text                     NOT NULL,
    content_text  text                     NOT NULL,
    excerpt       character varying(250)   NOT NULL,
    state         smallint                 NOT NULL DEFAULT 0 CHECK (state IN (0, 1, 2)),
    author_id     uuid                     NOT NULL,
    category_id   uuid                     NOT NULL,
    published_at  timestamp with time zone,
    created_at    timestamp with time zone NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at    timestamp with time zone NOT NULL DEFAULT CURRENT_TIMESTAMP,
    CONSTRAINT fk_post_category
        FOREIGN KEY (category_id) REFERENCES public.category (id) ON DELETE RESTRICT
);

COMMENT ON TABLE  public.post IS '博文';
COMMENT ON COLUMN public.post.title        IS '标题';
COMMENT ON COLUMN public.post.content_html IS '正文(HTML,展示用)';
COMMENT ON COLUMN public.post.content_text IS '正文纯文本(全文搜索用)';
COMMENT ON COLUMN public.post.excerpt      IS '摘要';
COMMENT ON COLUMN public.post.state        IS '状态(0草稿 1发布 2归档)';
COMMENT ON COLUMN public.post.author_id    IS '作者id';
COMMENT ON COLUMN public.post.category_id  IS '分类id';
COMMENT ON COLUMN public.post.published_at IS '发布时间';
COMMENT ON COLUMN public.post.created_at   IS '创建时间';
COMMENT ON COLUMN public.post.updated_at   IS '更新时间';

CREATE EXTENSION IF NOT EXISTS pgroonga SCHEMA public;

CREATE INDEX idx_post_title_pgroonga
    ON public.post USING pgroonga (title pgroonga_varchar_full_text_search_ops_v2)
    WHERE state = 1;

CREATE INDEX idx_post_content_text_pgroonga
    ON public.post USING pgroonga (content_text)
    WHERE state = 1;

CREATE INDEX idx_post_author_id          ON public.post (author_id);
CREATE INDEX idx_post_category_id        ON public.post (category_id);
CREATE INDEX idx_post_updated_at         ON public.post (updated_at DESC);
CREATE INDEX idx_post_state_published_at ON public.post (state, published_at DESC NULLS LAST);
