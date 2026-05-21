#[derive(Clone, Debug, PartialEq, Eq)]
pub enum SecurityEventType {
    TryLoginWithBadPwd,
    UserUnlocked, // 用户解锁后的事件才被纳入再次计算窗口内错误密码次数
    PasswordChanged,
}
