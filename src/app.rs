use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};

use crate::settings::Settings;
use crate::tools::stopwatch::Stopwatch;

// —— 主菜单 ——
pub const MAIN_MENU: [&str; 5] = ["Back", "Functions", "Settings", "About", "Quit"];

pub const MAIN_BACK: usize = 0;
pub const MAIN_FUNCTIONS: usize = 1;
pub const MAIN_SETTINGS: usize = 2;
pub const MAIN_ABOUT: usize = 3;
pub const MAIN_QUIT: usize = 4;

// —— 功能子菜单（先只放已实现的） ——
pub const FUNC_MENU: [&str; 1] = ["Stopwatch"];

pub const FUNC_STOPWATCH: usize = 0;

// —— 设置项 ——
pub const SETTINGS_ITEMS: [&str; 3] = ["Hour format", "Seconds", "Theme"];

pub enum Screen {
    Clock,
    Menu { selected: usize },
    Functions { selected: usize },
    Settings { selected: usize },
    About,
    Stopwatch,
}

pub struct App {
    pub screen: Screen,
    pub should_quit: bool,
    pub settings: Settings,
    pub stopwatch: Stopwatch,
}

impl App {
    pub fn new(settings: Settings) -> Self {
        Self {
            screen: Screen::Clock,
            should_quit: false,
            settings,
            stopwatch: Stopwatch::new(),
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
    }
}
