CREATE TABLE public.post_attachment (
    post_id       uuid        NOT NULL, -- 不同聚合不定义为FK
    attachment_id uuid        NOT NULL, -- 不同聚合不定义为FK
    created_at timestamp with time zone NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at timestamp with time zone,
    PRIMARY KEY (post_id, attachment_id)
);

-- Table comment
COMMENT ON TABLE public.post_attachment IS '博文附件关联表';

-- Column comments
CREATE INDEX idx_post_attachment_post ON public.post_attachment(post_id);
CREATE INDEX idx_post_attachment_attachment ON public.post_attachment(attachment_id);
