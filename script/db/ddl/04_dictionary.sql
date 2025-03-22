DROP TABLE public."dictionary";

CREATE TABLE public."dictionary"
(
    id        uuid        NOT NULL,
    value     varchar     NOT NULL, -- 值的字符串
    data_type varchar     NOT NULL, -- 数据类型
    "comment" varchar     NOT NULL, -- 字典项说明
    create_at timestamptz NOT NULL, -- 创建时间
    update_at timestamptz NULL,     -- 最后更新时间
    CONSTRAINT dictionary_pk PRIMARY KEY (id)
);
COMMENT ON TABLE public."dictionary" IS '字典';

-- Column comments

COMMENT ON COLUMN public."dictionary".value IS '值的字符串';
COMMENT ON COLUMN public."dictionary".data_type IS '数据类型';
COMMENT ON COLUMN public."dictionary"."comment" IS '字典项说明';
COMMENT ON COLUMN public."dictionary".create_at IS '创建时间';
COMMENT ON COLUMN public."dictionary".update_at IS '最后更新时间';