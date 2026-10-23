pub mod about;
pub mod clock;
pub mod menu;
pub mod settings;
pub mod tools;


use std::io::{self, stdout, Write};


use crossterm::cursor::MoveTo;
use crossterm::queue;
use crossterm::style::{Print, ResetColor};
use crossterm::terminal::{Clear, ClearType};

use crate::app::{App, Screen, FUNC_MENU, MAIN_MENU};
use crate::font;


const NOT_ENOUGH_ROOM: &str =
    "Not enough room for clock! Widen or heighten your terminal.";





/// 总入口：根据当前页面分发。这里只读 &App，不修改状态
pub fn draw(app: &App, term_w: u16, term_h: u16) -> io::Result<()> {
    match app.screen {
        Screen::Clock => clock::draw_clock(term_w, term_h, &app.settings),
        Screen::Menu { selected } => {
            menu::draw_menu(term_w, term_h, &MAIN_MENU, selected)
        }
        Screen::Functions { selected } => {
            menu::draw_menu(term_w, term_h, &FUNC_MENU, selected)
        }
        Screen::Settings { selected } => {
            settings::draw_settings(term_w, term_h, selected, &app.settings)
        }
        Screen::About => about::draw_about(term_w, term_h),
        Screen::Stopwatch => tools::draw_stopwatch(term_w, term_h, &app.stopwatch, &app.settings),
        Screen::Timer => tools::draw_timer(term_w, term_h, &app.timer, &app.settings),

    }
}


// 将文本渲染为字符画
fn render_text(text: &str, colon_on: bool) -> Vec<String> {
    let mut rows = vec![String::new(); font::GLYPH_HEIGHT];

    for ch in text.chars() {
        if let Some(glyph) = font::glyph_for(ch, colon_on) {
            for (i, row) in glyph.iter().enumerate() {
                debug_assert_eq!(
                    row.chars().count(),
                    font::GLYPH_WIDTH,
                    "字形第 {} 行宽度不是 {} 列: {:?}",
                    i,
                    font::GLYPH_WIDTH,
                    row
                );

                let line: String = row
                    .chars()
                    .map(|c| if c == '#' { font::FILL } else { ' ' })
                    .collect();
                rows[i].push_str(&line);
                rows[i].push_str(&" ".repeat(font::GLYPH_GAP));
            }
        }
        
    }

    for row in &mut rows {
        for _ in 0..font::GLYPH_GAP {
            row.pop();
        }
    }

    rows
}


// 绘制提示信息
fn draw_message(term_w: u16, term_h: u16, msg: &str) -> io::Result<()> {
    let msg_w = msg.chars().count() as u16;
    let x = term_w.saturating_sub(msg_w) / 2;
    let y = term_h / 2;

    let mut out = stdout();
    queue!(out, Clear(ClearType::All), ResetColor)?;
    queue!(out, MoveTo(x, y), Print(msg))?;
    out.flush()?;
    Ok(())
}


/// 多行文本：整块左对齐、整体居中
fn draw_lines(term_w: u16, term_h: u16, lines: &[String]) -> io::Result<()> {
    //block_w 是最长行的宽度
    let block_w = lines.iter().map(|l| l.chars().count()).max().unwrap_or(0);
    let block_h = lines.len();

    if (term_w as usize) < block_w || (term_h as usize) < block_h {
        return draw_message(term_w, term_h, NOT_ENOUGH_ROOM);
    }

    let x = (term_w as usize).saturating_sub(block_w) / 2;      // 只算一次
    let start_y = (term_h as usize).saturating_sub(block_h) / 2;

    let mut out = stdout();
    queue!(out, Clear(ClearType::All), ResetColor)?;                   //把 ResetColor加进 Clear那一批
    for (i, line) in lines.iter().enumerate() {
        queue!(out, MoveTo(x as u16, (start_y + i) as u16), Print(line))?;
    }
    out.flush()?;
    Ok(())
}


// 把攒好的整帧一次性发给终端
fn emit(frame: &[u8]) -> io::Result<()> {
    let mut out = stdout();
    out.write_all(frame)?;   // 整帧一次写出
    out.flush()              // 再 flush 一下，保证落到屏幕
}