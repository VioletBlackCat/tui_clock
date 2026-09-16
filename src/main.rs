mod app;
mod font;
mod ui;
mod theme;
mod config;

use std::io::{self, stdout};
use std::time::Duration;

use crossterm::cursor::{Hide, Show};
use crossterm::event::{self, Event, KeyEventKind};
use crossterm::execute;
use crossterm::terminal::{
    disable_raw_mode, enable_raw_mode, size, EnterAlternateScreen, LeaveAlternateScreen,
};



use app::{App, Screen};

fn main() -> io::Result<()> {
    
    let settings = config::load();
    
    // 首次运行：没有配置文件就生成一份，并把路径告诉用户
    if let Some(p) = config::create_if_missing(&settings) {
        eprintln!("Defalt configuration has been generated：{}", p.display());
        eprintln!("（Editable,takes effect on next startup）");
    }
    else {
        eprintln!("Config path: <configuration syntax error>");
    }
    
    //进入rawmode
    execute!(stdout(), EnterAlternateScreen, Hide)?;
    enable_raw_mode()?;

    let mut app = App::new(settings);

    loop {
        let (term_w, term_h) = size()?;
        ui::draw(&app, term_w, term_h)?;

        // 时钟页要每秒重画；菜单/设置页只在有事件时才变
        let timeout = match app.screen {
            Screen::Clock => Duration::from_millis(1000),
            _ => Duration::from_secs(3600),
        };

        if event::poll(timeout)? {
            if let Event::Key(key) = event::read()? {
                if key.kind == KeyEventKind::Press {
                    app.handle_key(key);
                }
            }
        }

        if app.should_quit {
            break;
        }
    }

    disable_raw_mode()?;
    execute!(stdout(), LeaveAlternateScreen, Show)?;
    
    //退出：写回配置（失败只提示，不影响退出）
    if let Err(e) = config::save(&app.settings) {
        eprintln!("Failed to save the configuration：{}", e);
    }


    Ok(())
}
