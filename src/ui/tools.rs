use std::io;

use crossterm::cursor::MoveTo;
use crossterm::queue;
use crossterm::style::{Print, ResetColor, SetForegroundColor};
use crossterm::terminal::{Clear, ClearType};

use crate::settings::Settings;
use crate::tools::{format_hms, stopwatch::Stopwatch, timer::Timer};

use super::{draw_message, emit, render_text, NOT_ENOUGH_ROOM};

/// 工具页通用版面：大号数字 + 状态行 + 按键提示
fn draw_tool_page(
    term_w: u16,
    term_h: u16,
    digits: &str,
    status: &str,
    hint: &str,
    settings: &Settings,
) -> io::Result<()> {
    let rows = render_text(digits, true);
    let content_w = rows[0].chars().count();
    let content_h = rows.len();

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

    let status_w = status.chars().count() as u16;
    queue!(
        &mut frame,
        MoveTo(term_w.saturating_sub(status_w) / 2, y + content_h as u16 + 1),
        SetForegroundColor(bottom_color),
        Print(status)
    )?;

    let hint_w = hint.chars().count() as u16;
    queue!(
        &mut frame,
        MoveTo(term_w.saturating_sub(hint_w) / 2, y + content_h as u16 + 2),
        ResetColor,
        Print(hint)
    )?;

    queue!(&mut frame, ResetColor)?;
    emit(&frame)
}

/// 秒表页
pub(super) fn draw_stopwatch(
    term_w: u16,
    term_h: u16,
    stopwatch: &Stopwatch,
    settings: &Settings,
) -> io::Result<()> {
    let digits = format_hms(stopwatch.elapsed());
    let status = if stopwatch.is_running() {
        "RUNNING"
    } else if stopwatch.elapsed().is_zero() {
        "READY"
    } else {
        "PAUSED"
    };

    draw_tool_page(
        term_w,
        term_h,
        &digits,
        status,
        "Space: start/pause   R: reset   Esc: back",
        settings,
    )
}

/// 计时器页
pub(super) fn draw_timer(
    term_w: u16,
    term_h: u16,
    timer: &Timer,
    settings: &Settings,
) -> io::Result<()> {
    let digits = format_hms(timer.remaining());

    let state = if timer.is_finished() {
        "DONE"
    } else if timer.is_running() {
        "RUNNING"
    } else if timer.remaining() == timer.total() {
        "READY"
    } else {
        "PAUSED"
    };
    let status = format!("{}   {}", state, timer.preset_label());

    draw_tool_page(
        term_w,
        term_h,
        &digits,
        &status,
        "Left/Right: preset   Space: start/pause   R: reset   Esc: back",
        settings,
    )
}
