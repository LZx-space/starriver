CREATE TABLE public.user_security_event
(
    id         uuid primary key         NOT NULL,
    user_id    uuid                     NOT NULL,
    event_type smallint                 NOT NULL,
    message    character varying(100)   NOT NULL,
    create_at  timestamp with time zone NOT NULL DEFAULT CURRENT_TIMESTAMP,
    update_at  timestamp with time zone
);

COMMENT ON TABLE public.user_security_event
    IS '用户安全事件';
