use std::time::Instant;

/// 实现延时以防止时差攻击
pub trait TimingAttackProtection {
    /// 依据方法处理开始时间，尝试延迟流程一个固定的时长
    /// # parameters
    /// - `start_at`: the instant when the fn process started
    fn fixed_duration_delay(&self, start_at: Instant) -> impl Future<Output = ()> + Send;
}
