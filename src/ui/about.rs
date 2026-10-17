//关于
use std::io;

use super::draw_lines;

// About文案 
const ABOUT_NAME: &str = env!("CARGO_PKG_NAME");
const ABOUT_VERSION: &str = env!("CARGO_PKG_VERSION");
const ABOUT_DESC: &str = "Hand-written with crossterm, no ratatui";
const ABOUT_AUTHOR: &str = "Violet_BlackCat";
const ABOUT_HINT: &str = "Esc - go back";
const ABOUT_LABEL_W: usize = 8;   // ≥ 最长标签 "Author" / "Config"

/// About 页：项目信息 + 配置文件路径
pub(super) fn draw_about(term_w: u16, term_h: u16) -> io::Result<()> {
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
