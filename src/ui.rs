use std::io::{self, stdout, Write};

use chrono::{Local, Timelike};
use crossterm::cursor::MoveTo;
use crossterm::queue;
use crossterm::style::{Print, ResetColor, SetForegroundColor};
use crossterm::terminal::{Clear, ClearType};

use crate::app::{App, Screen, MENU_ITEMS, Settings, SETTINGS_ITEMS};
use crate::font;


const NOT_ENOUGH_ROOM: &str =
    "Not enough room for clock! Widen or heighten your terminal.";

    //选项文字占的列数，选项每行固定占8+2=10列，用于draw_menu
const ITEM_W: usize = 10;
const LINE_W: usize = 2 + ITEM_W;

// 设置页：两列宽度
const LABEL_W: usize = 11;  // ≥ 最长项名 "Hour format"（11 字符）
const VALUE_W: usize = 7;   // ≥ 最长值文字 "classic"
const SETTINGS_LINE_W: usize = 2 + LABEL_W + 2 + VALUE_W;  // = 22

// About文案 
const ABOUT_NAME: &str = env!("CARGO_PKG_NAME");
const ABOUT_VERSION: &str = env!("CARGO_PKG_VERSION");
const ABOUT_DESC: &str = "Hand-written with crossterm, no ratatui";
const ABOUT_AUTHOR: &str = "Violet_BlackCat";
const ABOUT_HINT: &str = "Esc - go back";
const ABOUT_LABEL_W: usize = 8;   // ≥ 最长标签 "Author" / "Config"



/// 总入口：根据当前页面分发。这里只读 &App，不修改状态
pub fn draw(app: &App, term_w: u16, term_h: u16) -> io::Result<()> {
    match app.screen {
        Screen::Clock => draw_clock(term_w, term_h, &app.settings),
        Screen::Menu { selected } => draw_menu(term_w, term_h, selected),
        Screen::Settings { selected } => draw_settings(term_w, term_h, selected, &app.settings),
        Screen::About => draw_about(term_w, term_h) 
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


// 时钟页：取时间,渲染,判断窗口大小,绘制
fn draw_clock(term_w: u16, term_h: u16, settings: &Settings) -> io::Result<()> {
    let now = Local::now();
    let now_str = now.format(settings.time_pattern()).to_string();
    let colon_on = now.second() % 2 == 0;

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

/// 菜单页：整体居中，选中项前面带 '>'
pub fn draw_menu(term_w: u16, term_h: u16, selected: usize) -> io::Result<()> {

    let mut out = stdout();
    queue!(out, Clear(ClearType::All), ResetColor)?;

    let n = MENU_ITEMS.len();
    let block_h = 2 * n - 1;
    let start_y = (term_h as usize).saturating_sub(block_h) / 2;
    //空间不足时渲染提示语
    if (term_w as usize) < LINE_W || (term_h as usize) < block_h {
    return draw_message(term_w, term_h, NOT_ENOUGH_ROOM);
    }

    //x移出循环，只算一次，且基于常量 LINE_W
    let x = (term_w as usize).saturating_sub(LINE_W) / 2;

    for (i, item) in MENU_ITEMS.iter().enumerate() {
        let marker = if i == selected { '>' } else { ' ' };

        //^ 改成 <（左对齐），右侧自动补空格补满 8 列
        let label = format!("{} {:<width$}", marker, item, width = ITEM_W);

        //把「每行宽度」这件事钉死
        debug_assert_eq!(label.chars().count(), LINE_W);

        let y = start_y + i * 2;
        queue!(out, MoveTo(x as u16, y as u16), Print(&label))?;
    }

    out.flush()?;
    Ok(())
}

/// 设置页：左列项名、右列当前值，整块居中
fn draw_settings(term_w: u16, term_h: u16, selected: usize, settings: &Settings) -> io::Result<()> {
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

/// About 页：项目信息 + 配置文件路径
fn draw_about(term_w: u16, term_h: u16) -> io::Result<()> {
    let mut lines = vec![
        format!("{} v{}", ABOUT_NAME, ABOUT_VERSION),
        ABOUT_DESC.to_string(),
        String::new(),                                       // 空行分段
        format!("{:<lw$} {}", "Author", ABOUT_AUTHOR, lw = ABOUT_LABEL_W),
    ];

    // 配置文件路径,向config问，获取
    if let Some(p) = crate::config::config_path() {
        lines.push(format!("{:<lw$} {}", "Config", p.display(), lw = ABOUT_LABEL_W));
    }

    lines.push(String::new());
    lines.push(ABOUT_HINT.to_string());

    draw_lines(term_w, term_h, &lines)
}




// 把攒好的整帧一次性发给终端
fn emit(frame: &[u8]) -> io::Result<()> {
    let mut out = stdout();
    out.write_all(frame)?;   // 整帧一次写出
    out.flush()              // 再 flush 一下，保证落到屏幕
}