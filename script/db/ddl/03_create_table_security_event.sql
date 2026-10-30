CREATE TABLE public.security_event
(
    id          uuid PRIMARY KEY         NOT NULL,
    user_id     uuid                     NOT NULL,  -- 不同聚合不定义为FK
    username    character varying(20)    NOT NULL,  -- 事件发生时的用户名快照
    event_type  smallint                 NOT NULL,
    message     character varying(100)   NOT NULL,
    created_at  timestamp with time zone NOT NULL DEFAULT CURRENT_TIMESTAMP
);

-- Table comment
COMMENT ON TABLE public.security_event
    IS '用户安全事件';

COMMENT ON COLUMN public.security_event.username IS '事件发生时的用户名快照';

-- Index
CREATE INDEX idx_security_event_user_type_created ON public.security_event (user_id, event_type, created_at DESC, id DESC);
CREATE INDEX idx_security_event_created_id ON public.security_event (created_at DESC, id DESC);
