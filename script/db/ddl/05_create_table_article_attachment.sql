CREATE TABLE public.article_attachment
(
    id         uuid                     NOT NULL,
    article_id uuid                     NOT NULL,
    create_at  timestamp with time zone NOT NULL DEFAULT CURRENT_TIMESTAMP,
    update_at  timestamp with time zone,
    CONSTRAINT article_attachment_pkey PRIMARY KEY (id),
    CONSTRAINT article_attachment_fkey FOREIGN KEY (article_id)
        REFERENCES public.article (id) MATCH SIMPLE
        ON UPDATE NO ACTION
        ON DELETE CASCADE
);
COMMENT ON TABLE public.article_attachment IS '文章附件';

-- Column comments

COMMENT ON COLUMN public.article_attachment.id IS 'ID';
COMMENT ON COLUMN public.article_attachment.article_id IS '文章ID';
