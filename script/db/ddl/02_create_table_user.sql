CREATE TABLE public."user"
(
    id        uuid primary key         NOT NULL,
    username  character varying(100)   NOT NULL,
    password  character varying(100)   NOT NULL,
    create_at timestamp with time zone NOT NULL DEFAULT CURRENT_TIMESTAMP,
    update_at timestamp with time zone,
    CONSTRAINT user_unique UNIQUE (username)
);
COMMENT ON TABLE public."user" IS '用户';

-- Column comments
