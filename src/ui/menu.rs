//菜单
use std::io::{self, stdout, Write};

use crossterm::cursor::MoveTo;
use crossterm::queue;
use crossterm::style::{Print, ResetColor};
use crossterm::terminal::{Clear, ClearType};



use super::draw_message;
use super::NOT_ENOUGH_ROOM;

//选项文字占的列数，选项每行固定占8+2=10列，用于draw_menu
const ITEM_W: usize = 10;
const LINE_W: usize = 2 + ITEM_W;

/// 菜单页：整体居中，选中项前面带 '>'
pub(super) fn draw_menu(term_w: u16, term_h: u16, items: &[&str], selected: usize) -> io::Result<()> {

    let n = items.len();
    let block_h = 2 * n - 1;
    let start_y = (term_h as usize).saturating_sub(block_h) / 2;
    //空间不足时渲染提示语（draw_message 内部自己会清屏，这里不能再提前清，
    //否则这一帧会清两次屏；顺序与 ui/settings.rs 保持一致）
    if (term_w as usize) < LINE_W || (term_h as usize) < block_h {
    return draw_message(term_w, term_h, NOT_ENOUGH_ROOM);
    }

    let mut out = stdout();
    queue!(out, Clear(ClearType::All), ResetColor)?;

    //x移出循环，只算一次，且基于常量 LINE_W
    let x = (term_w as usize).saturating_sub(LINE_W) / 2;

    for (i, item) in items.iter().enumerate() {
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