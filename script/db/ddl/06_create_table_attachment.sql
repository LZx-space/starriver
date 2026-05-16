CREATE TABLE public.attachment
(
    id         uuid                     NOT NULL,
    file_name  character varying(50)    NOT NULL,
    created_at timestamp with time zone NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at timestamp with time zone,
    CONSTRAINT attachment_pkey PRIMARY KEY (id),
);
COMMENT ON TABLE public.attachment IS '附件';

-- Column comments

COMMENT ON COLUMN public.attachment.id IS 'ID';
COMMENT ON COLUMN public.attachment.file_name IS '文件名';
