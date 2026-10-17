//时钟
use std::io;

use chrono::{Local, Timelike};
use crossterm::cursor::MoveTo;
use crossterm::queue;
use crossterm::style::{Print, ResetColor, SetForegroundColor};
use crossterm::terminal::{Clear, ClearType};

use crate::app::Settings;
use super::{draw_message, emit, render_text, NOT_ENOUGH_ROOM};


// 时钟页：取时间,渲染,判断窗口大小,绘制
pub(super) fn draw_clock(term_w: u16, term_h: u16, settings: &Settings) -> io::Result<()> {
    let now = Local::now();
    let now_str = now.format(settings.time_pattern()).to_string();
    let colon_on = now.second().is_multiple_of(2);

    let rows = render_text(&now_str, colon_on);
    let content_w = rows[0].chars().count();
    let content_h = rows.len();

    // 12 小时制才要 AM/PM；24 制是 None，什么都不画
    let meridiem = if settings.hour_format.needs_meridiem() {
        Some(now.format("%p").to_string())      // "AM" / "PM"
    } else {
        None
    };

    // 竖直方向总共要吃多少行：时钟+(有 AM/PM 时）1行间隔+1行文字
    let extra_h = if meridiem.is_some() { 2 } else { 0 };
    let total_h = content_h + extra_h;

    // 窗口不够大 → 改画提示语（这就是从 main.rs 搬进来的那段）
    if (term_w as usize) < content_w || (term_h as usize) < total_h {
        return draw_message(term_w, term_h, NOT_ENOUGH_ROOM);
    }

    let x = term_w.saturating_sub(content_w as u16) / 2;
    let y = term_h.saturating_sub(total_h as u16) / 2;

    let span = (content_h - 1).max(1) as f32;   // 顶行到末行分几档
    let bottom_color = settings.theme.color_at(1.0);    // AM/PM 用它，和时钟底部同色

    
    let mut frame: Vec<u8> = Vec::with_capacity(4096);
    queue!(&mut frame, Clear(ClearType::All), ResetColor)?;                 // 清屏先排队
    
    for (i, row) in rows.iter().enumerate() {
        let t = i as f32 / span;                // 0.0（顶）→ 1.0（底）
        let color = settings.theme.color_at(t);

        queue!(&mut frame, MoveTo(x, y + i as u16), SetForegroundColor(color), Print(row))?;
    }
    //AM/PM
    if let Some(m) = &meridiem {
        let m_w = m.chars().count();
        let mx = (x as usize + content_w).saturating_sub(m_w) as u16;  //将AM/PM右对齐
        queue!(&mut frame, MoveTo(mx, y + content_h as u16 + 1), SetForegroundColor(bottom_color), Print(m))?;
    }
    //重置颜色
    queue!(&mut frame, ResetColor)?;  
    
    emit(&frame)
    
}