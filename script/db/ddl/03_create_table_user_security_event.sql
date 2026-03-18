CREATE TABLE public.user_security_event
(
    id         uuid                     NOT NULL,
    user_id    uuid                     NOT NULL,
    event_type smallint                 NOT NULL,
    message    character varying(100)   NOT NULL,
    create_at  timestamp with time zone NOT NULL DEFAULT CURRENT_TIMESTAMP,
    update_at  timestamp with time zone,
    CONSTRAINT user_security_event_pkey PRIMARY KEY (id),
    CONSTRAINT user_security_event_fkey FOREIGN KEY (user_id)
        REFERENCES public."user" (id) MATCH SIMPLE
        ON UPDATE NO ACTION
        ON DELETE CASCADE
);

COMMENT ON TABLE public.user_security_event
    IS '用户安全事件';
