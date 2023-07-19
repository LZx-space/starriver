-- public.article_tag definition

-- Drop table

-- DROP TABLE public.article_tag;

CREATE TABLE public.article_tag (
	id uuid NOT NULL,
	article_id uuid NOT NULL,
	tag_id uuid NOT NULL,
    create_at timestamptz NOT NULL DEFAULT CURRENT_TIMESTAMP,
    update_at timestamptz NULL DEFAULT CURRENT_TIMESTAMP,
	CONSTRAINT article_tag_pk PRIMARY KEY (id)
);