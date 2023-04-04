DROP TABLE if EXISTS article;
CREATE TABLE article (
    id
	code varchar NOT NULL,
	source_type bpchar(1) NOT NULL,
	business_code int4 NOT NULL,
	business varchar NOT NULL,
	reason varchar NOT NULL,
	CONSTRAINT exception_code_pk PRIMARY KEY (code)
);