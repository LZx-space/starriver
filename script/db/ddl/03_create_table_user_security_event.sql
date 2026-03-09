CREATE TABLE public.user_security_event
(
    id         uuid primary key         NOT NULL,
    type       smallint                 NOT NULL,
    is_success boolean                  NOT NULL DEFAULT false,
    message    character varying(100)   NOT NULL,
    create_at  timestamp with time zone NOT NULL DEFAULT CURRENT_TIMESTAMP,
    update_at  timestamp with time zone
);

COMMENT ON TABLE public.user_security_event
    IS '用户安全事件';
