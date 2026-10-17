use std::time::{Duration, Instant};

#[derive(Debug,Default)]
pub struct Ticker {
    //计时器的状态,Option<Instant>表示计时器是否在运行，Some(Instant)表示计时器正在运行，None表示计时器未运行
    //Instant表示一个时间点，Duration表示一段时间
    //之前暂停的时间会被累加到accumulated里，accumulated是Duration类型，表示计时器已经运行的总时间
    started_at: Option<Instant>,
    accumulated: Duration,
}

impl Ticker {
    //创建一个新的计时器，初始状态为未运行，累计时间为0（实际是一个零参数构造）
    pub fn new() -> Self {
        Self::default()
    }
    //计时器状态，看是否正在计时
    pub fn is_running(&self) -> bool {
    self.started_at.is_some()
    }
    //开始或继续计时，如果计时器已经在运行，就不做任何操作
    pub fn start(&mut self) {
        if self.started_at.is_none() {
            self.started_at = Some(Instant::now());
        }
    }
    //暂停时将这一段结算至 accumulated，并将 started_at 置为 None，表示计时器未运行 
    pub fn pause(&mut self) {
        if let Some(start) = self.started_at.take() {
            self.accumulated += start.elapsed();
        }
    }
    //计时器归零，停止
    pub fn reset(&mut self) {
        self.started_at = None;
        self.accumulated = Duration::ZERO;
    }
    //计时的累计时长
    pub fn elapsed(&self) -> Duration {
        let running = self.started_at.map_or(Duration::ZERO, |t| t.elapsed());
        self.accumulated + running 
    }

}

#[cfg(test)]
mod tests {
    use super::*;
    use std::thread::sleep;

    #[test]
    fn starts_stopped_and_at_zero() {
        let t = Ticker::new();
        assert!(!t.is_running());
        assert_eq!(t.elapsed(), Duration::ZERO);
    }

    #[test]
    fn pause_keeps_accumulated_time() {
        let mut t = Ticker::new();

        t.start();
        sleep(Duration::from_millis(20));
        t.pause();
        let first = t.elapsed();
        assert!(first >= Duration::from_millis(20));

        // 暂停期间不该增长
        sleep(Duration::from_millis(20));
        assert_eq!(t.elapsed(), first);

        // 继续之后应该接着涨
        t.start();
        sleep(Duration::from_millis(20));
        assert!(t.elapsed() >= first + Duration::from_millis(20));
    }

    #[test]
    fn reset_clears_everything() {
        let mut t = Ticker::new();
        t.start();
        sleep(Duration::from_millis(10));
        t.reset();

        assert!(!t.is_running());
        assert_eq!(t.elapsed(), Duration::ZERO);
    }
}
