use std::time::Instant;

/// 其函数实现延时以防止认证时的时差攻击
pub trait TimingAttackProtection {
    /// 依据认证开始时间，尝试延迟流程一个固定的时长
    /// # parameters
    /// - `authenticate_start_at`: the instant when the authentication process started
    fn fixed_duration_delay(
        &self,
        authenticate_start_at: Instant,
    ) -> impl Future<Output = ()> + Send;
}
