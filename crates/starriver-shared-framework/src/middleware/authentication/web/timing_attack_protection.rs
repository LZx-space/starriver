use core::time::Duration;
use std::time::Instant;

use tokio::time::sleep;

/// 实现延时以防止时差攻击
pub trait TimingAttackProtection {
    /// 依据方法处理开始时间，尝试延迟流程一个固定的时长
    /// # parameters
    /// - `start_at`: the instant when the fn process started
    fn fixed_duration_delay(&self, start_at: Instant) -> impl Future<Output = ()> + Send;
}

///////////////////////////////////////////////////////////////////////////////

/// 异步运行时为tokio时，使用tokio的sleep函数实现延时以防止认证时的时差攻击
pub struct TokioTimingAttackProtection {
    pub delay: Duration,
}

impl TimingAttackProtection for TokioTimingAttackProtection {
    async fn fixed_duration_delay(&self, authenticate_start_at: Instant) {
        let elapsed = authenticate_start_at.elapsed();
        let to_delay = self.delay.saturating_sub(elapsed);
        if Duration::ZERO.eq(&to_delay) {
            return;
        }
        sleep(to_delay).await;
    }
}

impl Default for TokioTimingAttackProtection {
    fn default() -> Self {
        Self {
            delay: Duration::from_millis(500),
        }
    }
}
