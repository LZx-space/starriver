CREATE TABLE public.post_attachment (
    post_id       uuid        NOT NULL,
    attachment_id uuid        NOT NULL,
    created_at timestamp with time zone NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at timestamp with time zone,
    PRIMARY KEY (post_id, attachment_id)
);
CREATE INDEX idx_post_attachment_post ON public.post_attachment(post_id);
CREATE INDEX idx_post_attachment_attachment ON public.post_attachment(attachment_id);
