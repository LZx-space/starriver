CREATE TABLE public.category
(
    id         uuid                     NOT NULL,
    name       character varying(10)    NOT NULL,
    create_at  timestamp with time zone NOT NULL DEFAULT CURRENT_TIMESTAMP,
    update_at  timestamp with time zone,
    CONSTRAINT category_pkey PRIMARY KEY (id)
);
COMMENT ON TABLE public.category IS '分类';

-- Column comments

COMMENT ON COLUMN public.category.id IS 'ID';
COMMENT ON COLUMN public.category.name IS '名称';
