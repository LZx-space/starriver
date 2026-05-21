CREATE TABLE public.attachment
(
    id         uuid PRIMARY KEY         NOT NULL,
    file_name  character varying(50)    NOT NULL,
    file_size  bigint                   NOT NULL,
    created_at timestamp with time zone NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at timestamp with time zone
);

COMMENT ON TABLE public.attachment IS '附件';

-- Column comments

COMMENT ON COLUMN public.attachment.id IS 'ID';
COMMENT ON COLUMN public.attachment.file_name IS '文件名';
COMMENT ON COLUMN public.attachment.file_size IS '文件大小';
