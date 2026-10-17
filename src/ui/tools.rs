use std::io;

use crossterm::cursor::MoveTo;
use crossterm::queue;
use crossterm::style::{Print, ResetColor, SetForegroundColor};
use crossterm::terminal::{Clear, ClearType};

use crate::settings::Settings;
use crate::tools::{format_hms, stopwatch::Stopwatch};

use super::{draw_message, emit, render_text, NOT_ENOUGH_ROOM};

/// 秒表页：大号数字 + 状态行 + 按键提示
pub(super) fn draw_stopwatch(
    term_w: u16,
    term_h: u16,
    stopwatch: &Stopwatch,
    settings: &Settings,
) -> io::Result<()> {
    let text = format_hms(stopwatch.elapsed());
    let rows = render_text(&text, true);      // 冒号常亮：秒表不该闪

    let content_w = rows[0].chars().count();
    let content_h = rows.len();

    let status = if stopwatch.is_running() {
        "RUNNING"
    } else if stopwatch.elapsed().is_zero() {
        "READY"
    } else {
        "PAUSED"
    };
    let hint = "Space: start/pause   R: reset   Esc: back";

    // 数字 7 行 + 空 1 行 + 状态 1 行 + 提示 1 行
    let total_h = content_h + 3;

    if (term_w as usize) < content_w || (term_h as usize) < total_h {
        return draw_message(term_w, term_h, NOT_ENOUGH_ROOM);
    }

    let x = term_w.saturating_sub(content_w as u16) / 2;
    let y = term_h.saturating_sub(total_h as u16) / 2;

    let span = (content_h - 1).max(1) as f32;
    let bottom_color = settings.theme.color_at(1.0);

    let mut frame: Vec<u8> = Vec::with_capacity(4096);
    queue!(&mut frame, Clear(ClearType::All))?;

    // 大号数字：沿用和时钟一样的竖直渐变
    for (i, row) in rows.iter().enumerate() {
        let t = i as f32 / span;
        let color = settings.theme.color_at(t);
        queue!(
            &mut frame,
            MoveTo(x, y + i as u16),
            SetForegroundColor(color),
            Print(row)
        )?;
    }

    // 状态行：和数字底部同色
    let status_w = status.chars().count() as u16;
    queue!(
        &mut frame,
        MoveTo(
            term_w.saturating_sub(status_w) / 2,
            y + content_h as u16 + 1
        ),
        SetForegroundColor(bottom_color),
        Print(status)
    )?;

    // 提示行：恢复默认色
    let hint_w = hint.chars().count() as u16;
    queue!(
        &mut frame,
        MoveTo(
            term_w.saturating_sub(hint_w) / 2,
            y + content_h as u16 + 2
        ),
        ResetColor,
        Print(hint)
    )?;

    queue!(&mut frame, ResetColor)?;

    emit(&frame)
}
