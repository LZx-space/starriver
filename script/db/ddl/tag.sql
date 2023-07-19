-- public.tag definition

-- Drop table

-- DROP TABLE public.tag;

CREATE TABLE public.tag (
	id uuid NOT NULL,
	"name" varchar NOT NULL,
	create_at timestamptz NOT NULL DEFAULT CURRENT_TIMESTAMP,
	update_at timestamptz NULL DEFAULT CURRENT_TIMESTAMP,
	CONSTRAINT tag_pk PRIMARY KEY (id)
);