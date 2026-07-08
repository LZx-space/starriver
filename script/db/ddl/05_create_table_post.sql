CREATE TABLE public.post
(
    id            uuid PRIMARY KEY         NOT NULL,
    title         character varying(50)    NOT NULL,  -- 对齐领域 Title::new(≤50)
    content       text                     NOT NULL,
    state         smallint                 NOT NULL DEFAULT 0 CHECK (state IN (0, 1, 2)), -- 0草稿 1发布 2归档
    search_vector tsvector,                           -- 搜索向量，sea-orm无法支持，靠触发器更新
    author_id     uuid                     NOT NULL,
    category_id   uuid                     NOT NULL,
    published_at  timestamp with time zone,
    created_at    timestamp with time zone NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at    timestamp with time zone,

    CONSTRAINT fk_post_author FOREIGN KEY (author_id)
        REFERENCES public."user"(id) MATCH SIMPLE
        ON UPDATE NO ACTION
        ON DELETE RESTRICT,

    CONSTRAINT fk_post_category FOREIGN KEY (category_id)
        REFERENCES public.category(id) MATCH SIMPLE
        ON UPDATE NO ACTION
        ON DELETE RESTRICT
);

COMMENT ON TABLE  public.post IS '博文';

COMMENT ON COLUMN public.post.title IS '标题';
COMMENT ON COLUMN public.post.content IS '正文';
COMMENT ON COLUMN public.post.state IS '状态';
COMMENT ON COLUMN public.post.search_vector IS '搜索向量';
COMMENT ON COLUMN public.post.author_id IS '作者id';
COMMENT ON COLUMN public.post.category_id IS '分类id';
COMMENT ON COLUMN public.post.published_at IS '发布时间';

CREATE INDEX idx_post_search             ON public.post USING gin(search_vector);
CREATE INDEX idx_post_author_id          ON public.post (author_id);
CREATE INDEX idx_post_category_id        ON public.post (category_id);
CREATE INDEX idx_post_updated_at         ON public.post (updated_at DESC NULLS LAST);
CREATE INDEX idx_post_state_published_at ON public.post (state, published_at DESC NULLS LAST);

-- EXTENSION: chinese text search

CREATE EXTENSION IF NOT EXISTS zhparser;
CREATE TEXT SEARCH CONFIGURATION zh_cfg (PARSER = zhparser);
ALTER  TEXT SEARCH CONFIGURATION zh_cfg ADD MAPPING FOR n,v,a,i,e,l WITH simple;

-- FUNCTION: public.update_post_search_vector()

CREATE FUNCTION update_post_search_vector() RETURNS trigger AS $$
BEGIN
  NEW.search_vector :=
    setweight(to_tsvector('zh_cfg', coalesce(NEW.title, '')), 'A') ||
    setweight(to_tsvector('zh_cfg', coalesce(NEW.content, '')), 'B');
  RETURN NEW;
END;
$$ LANGUAGE plpgsql;

-- TRIGGER: updating search_vector

CREATE TRIGGER trg_update_post_search_vector
    BEFORE INSERT OR UPDATE
    ON public.post
    FOR EACH ROW
    EXECUTE FUNCTION public.update_post_search_vector();
