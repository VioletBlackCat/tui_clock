use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};
use crate::theme::Theme;

pub const MENU_ITEMS: [&str; 4] = ["Back", "Settings", "About", "Quit"];
pub const SETTINGS_ITEMS: [&str; 3] = ["Hour format", "Seconds", "Theme"];

pub enum Screen {
    Clock,
    Menu { selected: usize },
    Settings { selected: usize},
    About,
}
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Settings {
    pub hour_format: HourFormat,
    pub show_seconds: bool,
    pub theme: Theme,
}

impl Default for Settings {
    fn default() -> Self {
        Self { hour_format: HourFormat::H24, show_seconds: true, theme: Theme::Classic }
    }
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
                KeyCode::Left | KeyCode::Right => match *selected {
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
}                                                   // ← impl 结束


//设置中的选项要先在app.rs中做出来，不能绑在设置的ui里面，否则一退出菜单就失效了
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
//时间格式设置
pub enum HourFormat { H24, H12 }

impl HourFormat{
    pub const ALL: [HourFormat; 2] = [HourFormat::H24, HourFormat::H12];
    
    //显示在Settings中的值文字
    pub fn label(self) -> &'static str {
        match self { HourFormat::H24 => "24H", HourFormat::H12 => "12H" }
    }
    // 从配置里的名字还原时制（大小写不敏感）
    pub fn from_name(s: &str) -> Option<Self> {
        Self::ALL.iter().copied().find(|f| f.label().eq_ignore_ascii_case(s))
    }
    
    //-> <-箭头切换
    pub fn toggled(self) -> Self {
        match self {
            HourFormat::H24 => HourFormat::H12,
            HourFormat::H12 => HourFormat::H24
        }
    }
    
    //12时制需要额外的AM/PM提示
    ////matches!(self, HourFormat::H12) 是个宏，等价于写一个只关心「是不是这个变体」的 match
    pub fn needs_meridiem(self) -> bool {
        matches!(self, HourFormat::H12)
    }
}





impl Settings {
    /// 当前设置下，时钟该用的格式串（4 种组合穷举）
    pub fn time_pattern(&self) -> &'static str {
        match (self.hour_format, self.show_seconds) {
            (HourFormat::H24, true)  => "%H:%M:%S",
            (HourFormat::H24, false) => "%H:%M",
            (HourFormat::H12, true)  => "%I:%M:%S",
            (HourFormat::H12, false) => "%I:%M",
        }
    }
}



