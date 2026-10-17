//设置
use std::io::{self, stdout, Write};

use crossterm::cursor::MoveTo;
use crossterm::queue;
use crossterm::style::{Print};
use crossterm::terminal::{Clear, ClearType};

use crate::app::{Settings, SETTINGS_ITEMS};
use super::{draw_message, NOT_ENOUGH_ROOM};

// 设置页：两列宽度
const LABEL_W: usize = 11;  // ≥ 最长项名 "Hour format"（11 字符）
const VALUE_W: usize = 7;   // ≥ 最长值文字 "classic"
const SETTINGS_LINE_W: usize = 2 + LABEL_W + 2 + VALUE_W;  // = 22

/// 设置页：左列项名、右列当前值，整块居中
pub(super) fn draw_settings(term_w: u16, term_h: u16, selected: usize, settings: &Settings) -> io::Result<()> {
    let n = SETTINGS_ITEMS.len();
    let block_h = 2 * n - 1;

    // 窗口不够大时渲染提示语
    if (term_w as usize) < SETTINGS_LINE_W || (term_h as usize) < block_h {
        return draw_message(term_w, term_h, NOT_ENOUGH_ROOM);
    }

    let start_y = (term_h as usize).saturating_sub(block_h) / 2;
    let x = (term_w as usize).saturating_sub(SETTINGS_LINE_W) / 2;

    let mut out = stdout();
    queue!(out, Clear(ClearType::All))?;

    for (i, item) in SETTINGS_ITEMS.iter().enumerate() {
        let marker = if i == selected { '>' } else { ' ' };

        // 「这一项现在的值是什么」——只有这里知道，以后加项就在这儿加分支
        let value = match i {
            0 => settings.hour_format.label(),
            1 => if settings.show_seconds { "on" } else { "off" },
            2 => settings.theme.label(),
            _ => "?",
        };

        // 两列各自补满：左列补到 LABEL_W，右列补到 VALUE_W
        let line = format!(
            "{} {:<lw$}  {:<vw$}",
            marker, item, value,
            lw = LABEL_W, vw = VALUE_W
        );
        debug_assert_eq!(line.chars().count(), SETTINGS_LINE_W);

        let y = start_y + i * 2;
        queue!(out, MoveTo(x as u16, y as u16), Print(&line))?;
    }

    out.flush()?;
    Ok(())
}