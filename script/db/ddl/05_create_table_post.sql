CREATE TABLE public.post
(
    id           uuid PRIMARY KEY         NOT NULL,
    title        character varying(50)    NOT NULL,  -- 对齐领域 Title::new(≤50)
    content      text                     NOT NULL,
    state        smallint                 NOT NULL,
    author_id    uuid                     NOT NULL,
    category_id  uuid                     NOT NULL,
    published_at timestamp with time zone,
    created_at   timestamp with time zone NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at   timestamp with time zone,

    CONSTRAINT post_author_fkey FOREIGN KEY (author_id)
        REFERENCES public."user"(id) MATCH SIMPLE
        ON UPDATE NO ACTION
        ON DELETE RESTRICT,

    CONSTRAINT post_category_fkey FOREIGN KEY (category_id)
        REFERENCES public.category(id) MATCH SIMPLE
        ON UPDATE NO ACTION
        ON DELETE RESTRICT
);

COMMENT ON TABLE public.post IS '博文';

COMMENT ON COLUMN public.post.title IS '标题';
COMMENT ON COLUMN public.post.content IS '正文';
COMMENT ON COLUMN public.post.state IS '状态';
COMMENT ON COLUMN public.post.author_id IS '作者';
COMMENT ON COLUMN public.post.category_id IS '分类';
COMMENT ON COLUMN public.post.published_at IS '发布时间';

CREATE INDEX idx_post_title        ON public.post (title);
CREATE INDEX idx_post_author_id    ON public.post (author_id);
CREATE INDEX idx_post_category_id  ON public.post (category_id);
CREATE INDEX idx_post_published_at ON public.post (published_at);
