-- public.tag definition

-- Drop table

-- DROP TABLE public.tag;

CREATE TABLE public.tag (
	id uuid NOT NULL,
	"name" varchar NOT NULL,
	create_time timestamptz NULL,
	CONSTRAINT tag_pk PRIMARY KEY (id)
);