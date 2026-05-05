use thiserror::Error;

#[derive(Debug, Error)]
pub enum DomainError {
    #[error("用户被锁定")]
    UserLocked,

    #[error("用户被禁用")]
    UserDisabled,

    #[error("用户名格式无效")]
    InvalidUsernameFormat,

    #[error("邮箱格式无效")]
    InvalidEmailFormat,

    #[error("密码错误")]
    BadPassword,

    #[error("密码格式无效")]
    InvalidPasswordFormat,

    #[error("密码编码失败：{0}")]
    PasswordEncoding(String),

    #[error("密码验证失败: {0}")]
    PasswordVerificationFailed(String),

    #[error("文章标题不能为空")]
    ArticleTitleIsEmpty,

    #[error("文章标题太长：{0}")]
    ArticleTitleTooLong(String),

    #[error("文章内容不能为空")]
    ArticleContentIsEmpty,

    #[error("文章内容太长：{0}")]
    ArticleContentTooLong(String),

    #[error("文章分类不能为空")]
    ArticleCategoryIsNone,

    #[error("文章分类名称太长：{0}")]
    ArticleCategoryTooLong(String),
}

#[derive(Debug, Error)]
pub enum RepositoryError {
    /// 实体未找到（业务可预期的错误）
    #[error("实体不存在: {0}")]
    NotFound(String),

    /// 唯一约束冲突，例如重复插入唯一键
    #[error("唯一约束冲突: {constraint} value={value}")]
    UniqueViolation {
        constraint: &'static str,
        value: &'static str,
    },

    /// 外键约束失败，引用的父记录不存在
    #[error("外键约束失败: {constraint} foreign_key={foreign_key}")]
    ForeignKeyViolation {
        constraint: &'static str,
        foreign_key: String,
    },

    /// 数据完整性错误（如字段过长、非空违反、类型不匹配）
    /// 注：最好由领域层提前校验，但数据库可能仍有触发
    #[error("数据完整性错误: {detail}")]
    DataIntegrity { detail: String },

    /// 乐观锁冲突（版本号或时间戳不匹配，需重试整个事务）
    #[error("乐观锁冲突, 实体已被修改: {entity} id={id}")]
    OptimisticLock { entity: &'static str, id: String },

    /// 死锁，通常是事务间资源竞争
    #[error("数据库死锁，事务需要重试")]
    Deadlock,

    /// 连接失败（网络、数据库宕机等可重试错误）
    #[error("数据库连接失败: {0}")]
    ConnectionFailed(String),

    /// 查询超时
    #[error("查询超时: {statement}")]
    Timeout { statement: String },

    /// 事务错误（如已经开始、提交失败）
    #[error("事务错误: {reason}")]
    Transaction { reason: String },

    /// 其他基础设施错误，保留底层细节但不泄露具体库（如 sqlx::Error）
    #[error("基础设施错误: {0}")]
    Infrastructure(String),

    /// 数据格式错误（如无效的日期、字符串等）
    #[error("数据格式错误: {0}")]
    BadData(String),

    /// 未知或未分类的错误
    #[error("未知的数据库错误: {message}")]
    Unknown { message: String },
}

#[derive(Debug, Error)]
pub enum PasswordEncoderError {
    /// 密码编码失败（如算法内部错误）
    #[error("密码编码失败: {0}")]
    EncodingFailed(String),

    /// 密码验证失败（原始密码与编码后的密码不匹配）
    #[error("密码验证失败: {0}")]
    VerificationFailed(String),

    /// 不支持的编码算法或格式
    #[error("不支持的密码编码算法或格式: {0}")]
    UnsupportedAlgorithm(String),

    /// 其他内部错误（如参数无效）
    #[error("内部错误: {0}")]
    InternalError(String),
}
