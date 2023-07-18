-- public.article definition

-- Drop table

-- DROP TABLE public.article;

CREATE TABLE public.article (
	id bigint NOT NULL,
	title varchar NULL,
	body text NULL,
	author_id varchar NULL,
	create_at timestamptz NOT NULL DEFAULT CURRENT_TIMESTAMP,
	update_at timestamptz NULL DEFAULT CURRENT_TIMESTAMP,
	CONSTRAINT article_pk PRIMARY KEY (id)
);
CREATE INDEX idx_article_title ON public.article USING btree (title);