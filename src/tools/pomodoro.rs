use std::time::Duration;

use super::timer::Timer;
//番茄钟的三个阶段：工作、短休息、长休息
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Phase {
    Work,
    ShortBreak,
    LongBreak,
}

impl Phase {
    pub fn label(self) -> &'static str {
        match self {
            Phase::Work => "WORK",
            Phase::ShortBreak => "SHORT BREAK",
            Phase::LongBreak => "LONG BREAK",
        }
    }

    pub fn duration(self) -> Duration {
        match self {
            Phase::Work => Duration::from_secs(25 * 60),
            Phase::ShortBreak => Duration::from_secs(5 * 60),
            Phase::LongBreak => Duration::from_secs(15 * 60),
        }
    }
}

#[derive(Debug)]
pub struct Pomodoro {
    timer: Timer,
    phase: Phase,
    completed_works: u32,
}

impl Default for Pomodoro {
    fn default() -> Self {
        Self::new()
    }
}

impl Pomodoro {
    pub fn new() -> Self {
        Self {
            timer: Timer::with_total(Phase::Work.duration()),
            phase: Phase::Work,
            completed_works: 0,
        }
    }

    pub fn phase_label(&self) -> &'static str {
        self.phase.label()
    }

    pub fn completed_works(&self) -> u32 {
        self.completed_works
    }

    pub fn is_running(&self) -> bool {
        self.timer.is_running()
    }

    pub fn remaining(&self) -> Duration {
        self.timer.remaining()
    }

    pub fn total(&self) -> Duration {
        self.timer.total()
    }

    //空格：开始 / 暂停 / 到点后重开
    pub fn toggle(&mut self) {
        self.timer.toggle();
    }

    //R：重置当前阶段（阶段和计数都不变）
    pub fn reset(&mut self) {
        self.timer.reset();
    }

    //N：跳过当前阶段，直接进入下一阶段
    pub fn skip(&mut self) {
        self.advance_phase();
    }

     //每轮循环调用：到点就切阶段，并返回 true（提醒一次）
    pub fn poll_finished(&mut self) -> bool {
        if self.timer.poll_finished() {
            self.advance_phase();
            return true;
        }
        false
    }

    //阶段切换的全部规则
    fn advance_phase(&mut self) {
        self.phase = match self.phase {
            Phase::Work => {
                self.completed_works += 1;
                if self.completed_works % 4 == 0 {
                    Phase::LongBreak
                } else {
                    Phase::ShortBreak
                }
            }
            Phase::ShortBreak => Phase::Work,
            Phase::LongBreak => {
                self.completed_works = 0;   // 长休结束 = 新一轮
                Phase::Work
            }
        };

        // 换阶段 = 换一个全新的倒计时（未启动）
        self.timer = Timer::with_total(self.phase.duration());
    }

}

#[cfg(test)]
mod tests {
    use super::*;
    use std::thread::sleep;

    #[test]
    fn work_break_cycle() {
        let mut p = Pomodoro::new();
        assert_eq!(p.phase_label(), "WORK");

        // 第 1 个工作结束
        p.skip();
        assert_eq!(p.phase_label(), "SHORT BREAK");
        p.skip();
        assert_eq!(p.phase_label(), "WORK");

        // 第 2 个
        p.skip();
        assert_eq!(p.phase_label(), "SHORT BREAK");
        p.skip();
        assert_eq!(p.phase_label(), "WORK");

        // 第 3 个
        p.skip();
        assert_eq!(p.phase_label(), "SHORT BREAK");
        p.skip();
        assert_eq!(p.phase_label(), "WORK");

        // 第 4 个 → 长休
        p.skip();
        assert_eq!(p.phase_label(), "LONG BREAK");
        assert_eq!(p.completed_works(), 4);

        // 长休结束 → 新一轮，计数归零
        p.skip();
        assert_eq!(p.phase_label(), "WORK");
        assert_eq!(p.completed_works(), 0);
    }

    #[test]
    fn skip_resets_the_countdown() {
        let mut p = Pomodoro::new();
        p.toggle();
        sleep(Duration::from_millis(10));
        p.skip();

        assert_eq!(p.remaining(), p.total());   // 新阶段从满格开始
        assert!(!p.is_running());               // 而且没有自动开跑
    }

    #[test]
    fn finished_advances_phase_once() {
        let mut p = Pomodoro::new();
        // 测试专用：把当前阶段的倒计时换成 20ms
        p.timer = Timer::with_total(Duration::from_millis(20));
        p.toggle();

        sleep(Duration::from_millis(30));

        assert!(p.poll_finished());              // 第一次：提醒 + 切阶段
        assert_eq!(p.phase_label(), "SHORT BREAK");
        assert!(!p.poll_finished());             // 第二次：不再提醒
    }
}
