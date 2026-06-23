CREATE TABLE public."user"
(
    id         uuid PRIMARY KEY         NOT NULL,
    username   character varying(20)    NOT NULL,
    password   character varying(128)   NOT NULL,
    email      character varying(254)   NOT NULL,
    state      smallint                 NOT NULL DEFAULT 0,
    bad_password_window_start timestamp with time zone,
    bad_password_attempts     smallint  NOT NULL DEFAULT 0,
    created_at timestamp with time zone NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at timestamp with time zone,
    CONSTRAINT uq_username UNIQUE (username),
    CONSTRAINT uq_email UNIQUE (email)
);
COMMENT ON TABLE public."user" IS '用户';

-- Column comments
COMMENT ON COLUMN public."user".bad_password_window_start IS '密码错误计数窗口开始时间';
COMMENT ON COLUMN public."user".bad_password_attempts IS '窗口内密码错误尝试次数';
