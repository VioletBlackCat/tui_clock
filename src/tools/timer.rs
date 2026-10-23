use std::time::Duration;

use super::ticker::Ticker;

// 预设时长：(秒数, 显示名)
pub const PRESETS: [(u64, &str); 5] = [
    (10, "10s"),
    (60, "1m"),
    (300, "5m"),
    (900, "15m"),
    (1500, "25m"),
];

#[derive(Debug)]
pub struct Timer {
    ticker: Ticker,
    total: Duration,
    notified: bool,
}

impl Default for Timer {
    fn default() -> Self {
        Self::new()
    }
}

impl Timer {
    //默认5分钟
    pub fn new() -> Self {
        Self::with_total(Duration::from_secs(300))
    }

    /// 指定总时长（测试和以后的「自定义时长」都用得上）
    pub fn with_total(total: Duration) -> Self {
        Self {
            ticker: Ticker::new(),
            total,
            notified: false,
        }
    }

    pub fn is_running(&self) -> bool {
        self.ticker.is_running()
    }

    pub fn total(&self) -> Duration {
        self.total
    }

    //还剩多少（到 0 就停在那儿，不会变成负数）
    pub fn remaining(&self) -> Duration {
        self.total.saturating_sub(self.ticker.elapsed())
    }

    //是否已经到点
    pub fn is_finished(&self) -> bool {
        self.ticker.elapsed() >= self.total
    }

    //空格：开始 / 暂停；已经到点则重新开始
    pub fn toggle(&mut self) {
        if self.ticker.is_running() {
            self.ticker.pause();
        } else if self.is_finished() {
            self.reset();
            self.ticker.start();
        } else {
            self.ticker.start();
        }
    }

    // R：归零并停止（总时长不变）
    pub fn reset(&mut self) {
        self.ticker.reset();
        self.notified = false;
    }

    //每轮循环调用一次，检查是否到点
    //第一次跨过终点时返回 true（同时自动暂停、只提醒一次）。
    pub fn poll_finished(&mut self) -> bool {
        if !self.notified && self.is_finished() {
            self.notified = true;
            self.ticker.pause();
            return true;
        }
        false
    }

    // 当前总时长对应的预设名（找不到就 "custom"）
    pub fn preset_label(&self) -> &'static str {
        for entry in PRESETS {
            if Duration::from_secs(entry.0) == self.total {
                return entry.1;
            }
        }
        "custom"
    }

    // →：下一个预设（会重置计时）
    pub fn next_preset(&mut self) {
        let i = (self.preset_index() + 1) % PRESETS.len();
        self.set_total(Duration::from_secs(PRESETS[i].0));
    }

    // ←：上一个预设（会重置计时）
    pub fn prev_preset(&mut self) {
        let i = (self.preset_index() + PRESETS.len() - 1) % PRESETS.len();
        self.set_total(Duration::from_secs(PRESETS[i].0));
    }

    // 当前总时长在 PRESETS 里的下标（找不到就当 0）
    fn preset_index(&self) -> usize {
        for (i, entry) in PRESETS.iter().enumerate() {
            if Duration::from_secs(entry.0) == self.total {
                return i;
            }
        }
        0
    }

    // 换总时长：顺手重置，保证「换档 = 重新开始」
    fn set_total(&mut self, total: Duration) {
        self.total = total;
        self.reset();
    }

}

#[cfg(test)]
mod tests {
    use super::*;
    use std::thread::sleep;

    #[test]
    fn presets_cycle_forward_and_back() {
        let mut t = Timer::with_total(Duration::from_secs(60));
        assert_eq!(t.preset_label(), "1m");

        t.next_preset();
        assert_eq!(t.preset_label(), "5m");

        t.prev_preset();
        assert_eq!(t.preset_label(), "1m");

        t.prev_preset();
        assert_eq!(t.preset_label(), "10s");   // 1m 往回是 10s
    }

    #[test]
    fn finishes_and_notifies_once() {
        let mut t = Timer::with_total(Duration::from_millis(20));
        t.toggle();
        assert!(t.is_running());

        sleep(Duration::from_millis(30));

        assert!(t.is_finished());
        assert_eq!(t.remaining(), Duration::ZERO);
        assert!(t.poll_finished());      // 第一次：该提醒
        assert!(!t.poll_finished());     // 第二次：不许再提醒
        assert!(!t.is_running());        // 到点自动暂停
    }
}
