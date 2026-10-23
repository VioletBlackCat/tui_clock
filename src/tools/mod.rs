//! 与终端无关的计时工具：秒表、计时器、番茄钟。

pub mod stopwatch;
pub mod ticker;
pub mod timer;

use std::time::Duration;

/// 把时长格式化成 `HH:MM:SS`
pub fn format_hms(d: Duration) -> String {
    let total = d.as_secs();
    let h = total / 3600;
    let m = (total % 3600) / 60;
    let s = total % 60;
    format!("{:02}:{:02}:{:02}", h, m, s)
}

#[cfg(test)]
mod tests {
    use super::format_hms;
    use std::time::Duration;

    #[test]
    fn formats_hms() {
        assert_eq!(format_hms(Duration::ZERO), "00:00:00");
        assert_eq!(format_hms(Duration::from_secs(61)), "00:01:01");
        assert_eq!(format_hms(Duration::from_secs(3661)), "01:01:01");
    }
}
