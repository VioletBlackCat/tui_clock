//菜单
use std::io::{self, stdout, Write};

use crossterm::cursor::MoveTo;
use crossterm::queue;
use crossterm::style::{Print, ResetColor};
use crossterm::terminal::{Clear, ClearType};



use super::draw_message;
use super::NOT_ENOUGH_ROOM;

//选项文字的最小列数（原有观感：8 列文字 + 2 列标记/间隔）
const MIN_ITEM_W: usize = 8;
const ITEM_PAD: usize = 2;      // 选中标记 '>' + 一个空格

/// 菜单页：整体居中，选中项前面带 '>'
pub(super) fn draw_menu(term_w: u16, term_h: u16, items: &[&str], selected: usize) -> io::Result<()> {

    //选项列宽按「最长项名」现算，不再写死常量
    //写死的话，以后加一个更长的菜单项就会：debug 下 debug_assert 直接 panic，
    //release 下不 panic 但整块菜单错位（x 是按旧宽度算的）
    let item_w = items
        .iter()
        .map(|s| s.chars().count())
        .max()
        .unwrap_or(0)
        .max(MIN_ITEM_W);
    let line_w = ITEM_PAD + item_w;

    let n = items.len();
    let block_h = 2 * n - 1;
    let start_y = (term_h as usize).saturating_sub(block_h) / 2;
    //空间不足时渲染提示语（draw_message 内部自己会清屏，这里不能再提前清，
    //否则这一帧会清两次屏；顺序与 ui/settings.rs 保持一致）
    if (term_w as usize) < line_w || (term_h as usize) < block_h {
    return draw_message(term_w, term_h, NOT_ENOUGH_ROOM);
    }

    let mut out = stdout();
    queue!(out, Clear(ClearType::All), ResetColor)?;

    //x移出循环，只算一次，且基于 line_w
    let x = (term_w as usize).saturating_sub(line_w) / 2;

    for (i, item) in items.iter().enumerate() {
        let marker = if i == selected { '>' } else { ' ' };

        //左对齐，右侧自动补空格补满 item_w 列
        let label = format!("{} {:<width$}", marker, item, width = item_w);

        //把「每行宽度」这件事钉死
        debug_assert_eq!(label.chars().count(), line_w);

        let y = start_y + i * 2;
        queue!(out, MoveTo(x as u16, y as u16), Print(&label))?;
    }

    out.flush()?;
    Ok(())
}