CREATE TABLE public.blog_attachment
(
    id         uuid                     NOT NULL,
    blog_id    uuid                     NOT NULL,
    create_at  timestamp with time zone NOT NULL DEFAULT CURRENT_TIMESTAMP,
    update_at  timestamp with time zone,
    CONSTRAINT blog_attachment_pkey PRIMARY KEY (id),
    CONSTRAINT blog_attachment_fkey FOREIGN KEY (blog_id)
        REFERENCES public."blog" (id) MATCH SIMPLE
        ON UPDATE NO ACTION
        ON DELETE CASCADE
);
COMMENT ON TABLE public.blog_attachment IS '博客附件';

-- Column comments

COMMENT ON COLUMN public.blog_attachment.id IS 'ID';
COMMENT ON COLUMN public.blog_attachment.blog_id IS '博客ID';
