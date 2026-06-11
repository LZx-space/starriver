CREATE TABLE public."user"
(
    id         uuid PRIMARY KEY         NOT NULL,
    username   character varying(20)    NOT NULL,
    password   character varying(128)   NOT NULL,
    email      character varying(254)   NOT NULL,
    state      smallint                 NOT NULL DEFAULT 0,
    created_at timestamp with time zone NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at timestamp with time zone,
    CONSTRAINT uq_username UNIQUE (username),
    CONSTRAINT uq_email UNIQUE (email)
);
COMMENT ON TABLE public."user" IS '用户';

-- Column comments
