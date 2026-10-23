use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};

use crate::settings::Settings;
use crate::tools::stopwatch::Stopwatch;
use crate::tools::timer::Timer;


// —— 主菜单 ——
pub const MAIN_MENU: [&str; 5] = ["Back", "Functions", "Settings", "About", "Quit"];

pub const MAIN_BACK: usize = 0;
pub const MAIN_FUNCTIONS: usize = 1;
pub const MAIN_SETTINGS: usize = 2;
pub const MAIN_ABOUT: usize = 3;
pub const MAIN_QUIT: usize = 4;

// —— 功能子菜单（先只放已实现的） ——
pub const FUNC_MENU: [&str; 2] = ["Stopwatch", "Timer"];

pub const FUNC_STOPWATCH: usize = 0;
pub const FUNC_TIMER: usize = 1;

// —— 设置项 ——
pub const SETTINGS_ITEMS: [&str; 3] = ["Hour format", "Seconds", "Theme"];

pub enum Screen {
    Clock,
    Menu { selected: usize },
    Functions { selected: usize },
    Settings { selected: usize },
    About,
    Stopwatch,
    Timer,
}

pub struct App {
    pub screen: Screen,
    pub should_quit: bool,
    pub settings: Settings,
    pub stopwatch: Stopwatch,
    pub timer: Timer,
}

impl App {
    pub fn new(settings: Settings) -> Self {
        Self {
            screen: Screen::Clock,
            should_quit: false,
            settings,
            stopwatch: Stopwatch::new(),
            timer: Timer::new(),
        }
    }

    pub fn handle_key(&mut self, key: KeyEvent) {
        // Ctrl+C 随时退出
        if key.modifiers.contains(KeyModifiers::CONTROL) && key.code == KeyCode::Char('c') {
            self.should_quit = true;
            return;
        }

        // 全局快捷键：任何页面都能直达功能
        if let KeyCode::Char('s') | KeyCode::Char('S') = key.code {
            self.screen = Screen::Stopwatch;
            return;
        }

        if let KeyCode::Char('t') | KeyCode::Char('T') = key.code {
            self.screen = Screen::Timer;
            return;
        }

        let mut next: Option<Screen> = None;

        match &mut self.screen {
            Screen::Clock => match key.code {
                KeyCode::Esc => next = Some(Screen::Menu { selected: MAIN_BACK }),
                KeyCode::Char('q') | KeyCode::Char('Q') => self.should_quit = true,
                _ => {}
            },
            Screen::Menu { selected } => match key.code {
                KeyCode::Esc => next = Some(Screen::Clock),
                KeyCode::Up => {
                    *selected = if *selected == 0 {
                        MAIN_MENU.len() - 1
                    } else {
                        *selected - 1
                    };
                }
                KeyCode::Down => {
                    *selected = (*selected + 1) % MAIN_MENU.len();
                }
                KeyCode::Enter => match *selected {
                    MAIN_BACK => next = Some(Screen::Clock),
                    MAIN_FUNCTIONS => next = Some(Screen::Functions { selected: 0 }),
                    MAIN_SETTINGS => next = Some(Screen::Settings { selected: 0 }),
                    MAIN_ABOUT => next = Some(Screen::About),
                    MAIN_QUIT => self.should_quit = true,
                    _ => {}
                },
                _ => {}
            },
            Screen::Functions { selected } => match key.code {
                KeyCode::Esc => next = Some(Screen::Menu { selected: MAIN_FUNCTIONS }),
                KeyCode::Up => {
                    *selected = if *selected == 0 {
                        FUNC_MENU.len() - 1
                    } else {
                        *selected - 1
                    };
                }
                KeyCode::Down => {
                    *selected = (*selected + 1) % FUNC_MENU.len();
                }
                KeyCode::Enter => match *selected {
                    FUNC_STOPWATCH => next = Some(Screen::Stopwatch),
                    FUNC_TIMER => next = Some(Screen::Timer),
                    _ => {}
                },
                _ => {}
            },
            Screen::Stopwatch => match key.code {
                KeyCode::Esc => next = Some(Screen::Functions { selected: FUNC_STOPWATCH }),
                KeyCode::Char(' ') => self.stopwatch.toggle(),
                KeyCode::Char('r') | KeyCode::Char('R') => self.stopwatch.reset(),
                _ => {}
            },

            Screen::Timer => match key.code {
                KeyCode::Esc => next = Some(Screen::Functions { selected: FUNC_TIMER }),
                KeyCode::Char(' ') => self.timer.toggle(),
                KeyCode::Char('r') | KeyCode::Char('R') => self.timer.reset(),
                KeyCode::Left => self.timer.prev_preset(),
                KeyCode::Right => self.timer.next_preset(),
                _ => {}
            },

                Screen::Settings { selected } => match key.code {
                KeyCode::Esc => next = Some(Screen::Menu { selected: MAIN_SETTINGS }),
                KeyCode::Up => {
                    *selected = if *selected == 0 {
                        SETTINGS_ITEMS.len() - 1
                    } else {
                        *selected - 1
                    };
                }
                KeyCode::Down => {
                    *selected = (*selected + 1) % SETTINGS_ITEMS.len();
                }
                KeyCode::Left | KeyCode::Right => match *selected {
                    0 => self.settings.hour_format = self.settings.hour_format.toggled(),
                    1 => self.settings.show_seconds = !self.settings.show_seconds,
                    2 => {
                        self.settings.theme = if key.code == KeyCode::Right {
                            self.settings.theme.next()
                        } else {
                            self.settings.theme.prev()
                        };
                    }
                    _ => {}
                },
                _ => {}
            },

            Screen::About => {
                if key.code == KeyCode::Esc {
                    next = Some(Screen::Menu { selected: MAIN_ABOUT });
                }
            }
        }

        if let Some(s) = next {
            self.screen = s;
        }
    }

    //时间驱动的更新：每轮循环调用一次。
    //返回 true 表示这一轮需要提醒（目前是计时器到点）。
    pub fn update(&mut self) -> bool {
        self.timer.poll_finished()
    }

}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn menu_constants_match_items() {
        assert_eq!(MAIN_MENU[MAIN_BACK], "Back");
        assert_eq!(MAIN_MENU[MAIN_FUNCTIONS], "Functions");
        assert_eq!(MAIN_MENU[MAIN_SETTINGS], "Settings");
        assert_eq!(MAIN_MENU[MAIN_ABOUT], "About");
        assert_eq!(MAIN_MENU[MAIN_QUIT], "Quit");
        assert_eq!(FUNC_MENU[FUNC_STOPWATCH], "Stopwatch");
        assert_eq!(FUNC_MENU[FUNC_TIMER], "Timer");

    }
}
