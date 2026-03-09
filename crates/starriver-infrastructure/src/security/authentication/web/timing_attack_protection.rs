use std::time::Duration;

/// 其函数实现延时以防止认证时的时差攻击
pub trait TimingAttackProtection {
    /// # parameters
    /// - `already_spend`: the duration already spent on the authentication process
    fn delay(&self, already_spend: Duration) -> impl Future<Output = ()> + Send;
}
