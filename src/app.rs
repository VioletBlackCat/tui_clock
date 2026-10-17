use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};
use crate::settings::Settings;


pub const MENU_ITEMS: [&str; 4] = ["Back", "Settings", "About", "Quit"];
pub const SETTINGS_ITEMS: [&str; 3] = ["Hour format", "Seconds", "Theme"];

pub enum Screen {
    Clock,
    Menu { selected: usize },
    Settings { selected: usize},
    About,
}

pub struct App {
    pub screen: Screen,
    pub should_quit: bool,
    pub settings: Settings,
}
//handle_key 通过 &mut self.screen 这个可变借用，「看」当前状态、算出「下一个状态」，用 next 攒着 → match 结束、借用归还 → 写回 self.screen 
impl App {
    pub fn new(settings:Settings) -> Self {
        Self { 
            screen: Screen::Clock,
            should_quit: false,
            settings,
        }
    }

    pub fn handle_key(&mut self, key: KeyEvent) {
        if key.modifiers.contains(KeyModifiers::CONTROL) && key.code == KeyCode::Char('c') {
            self.should_quit = true;
            return;
        }

        let mut next: Option<Screen> = None;

        match &mut self.screen {
            Screen::Clock => match key.code {
                KeyCode::Esc => next = Some(Screen::Menu { selected: 0 }),
                KeyCode::Char('q') | KeyCode::Char('Q') => self.should_quit = true,
                _ => {}
            },
            Screen::Menu { selected } => match key.code {
                KeyCode::Esc => next = Some(Screen::Clock),
                KeyCode::Up => {
                    *selected = if *selected == 0 { MENU_ITEMS.len() - 1 } else { *selected - 1 };
                }
                KeyCode::Down => {
                    *selected = (*selected + 1) % MENU_ITEMS.len();
                }
                
                
                KeyCode::Enter => match *selected {
                    0 => next = Some(Screen::Clock),
                    1 => next = Some(Screen::Settings { selected:0}),
                    2 => next = Some(Screen::About),
                    3 => self.should_quit = true,
                    _ => {}
                },
                _ => {}
            },
            Screen::Settings {selected } => match key.code {
                KeyCode::Esc => next = Some(Screen::Menu { selected: 1 }), // 回菜单时游标停在 Settings
                KeyCode::Up => {
                    *selected = if *selected == 0 { SETTINGS_ITEMS.len() - 1 } else { *selected - 1 };
                }
                KeyCode::Down => {
                    *selected = (*selected + 1) % SETTINGS_ITEMS.len();
                }
                KeyCode::Left => match *selected {
                    0 => self.settings.hour_format = self.settings.hour_format.toggled(),
                    1 => self.settings.show_seconds = !self.settings.show_seconds,
                    2 => self.settings.theme = self.settings.theme.prev(),
                    _ => {}
                },
                KeyCode::Right => match *selected {
                    0 => self.settings.hour_format = self.settings.hour_format.toggled(),
                    1 => self.settings.show_seconds = !self.settings.show_seconds,
                    2 => self.settings.theme = self.settings.theme.next(),
                    _ => {}
                },
            _ => {}
            }
            Screen::About => {
                if key.code == KeyCode::Esc {
                    next = Some(Screen::Menu { selected: 2 });
                }
            }
        }

        if let Some(s) = next {
            self.screen = s;
        }
    }
}





