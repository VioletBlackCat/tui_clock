use std::time::Duration;

use super::ticker::Ticker;

/// 秒表：只管「开始 / 暂停 / 归零」这套规则，不关心怎么显示。
#[derive(Debug, Default)]
pub struct Stopwatch {
    ticker: Ticker,
}

impl Stopwatch {
    pub fn new() -> Self {
        Self { ticker: Ticker::new() }
    }

    /// 空格键：在「跑」和「停」之间切换
    pub fn toggle(&mut self) {
        if self.ticker.is_running() {
            self.ticker.pause();
        } else {
            self.ticker.start();
        }
    }

    /// R 键：归零并停止
    pub fn reset(&mut self) {
        self.ticker.reset();
    }

    pub fn is_running(&self) -> bool {
        self.ticker.is_running()
    }

    pub fn elapsed(&self) -> Duration {
        self.ticker.elapsed()
    }
}

#[cfg(test)]
mod tests {
    use super::Stopwatch;
    use std::time::Duration;
    use std::thread::sleep;

    #[test]
    fn toggle_starts_and_pauses() {
        let mut s = Stopwatch::new();
        assert!(!s.is_running());

        s.toggle();                        // 开始
        assert!(s.is_running());
        sleep(Duration::from_millis(10));

        s.toggle();                        // 暂停
        assert!(!s.is_running());

        let paused_at = s.elapsed();
        sleep(Duration::from_millis(10));
        assert_eq!(s.elapsed(), paused_at);   // 暂停期间不涨

        s.reset();
        assert_eq!(s.elapsed(), Duration::ZERO);
    }
}
