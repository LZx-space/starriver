#[derive(Clone, Debug, PartialEq, Eq)]
pub enum SecurityEventType {
    TryLoginWithBadPwd,
    PasswordChanged,
}
