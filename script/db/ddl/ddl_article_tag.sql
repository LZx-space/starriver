-- public.article_rel_tag definition

-- Drop table

-- DROP TABLE public.article_rel_tag;

CREATE TABLE public.article_rel_tag (
	id uuid NOT NULL,
	article_id uuid NULL,
	tag_id uuid NULL
);