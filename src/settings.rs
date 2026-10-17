//设置
use crate::theme::Theme;

//设置项与状态逻辑必须放在数据模型层（settings.rs），不能放进 ui/settings.rs
//否则绘制模块就成了状态的唯一持有者：一退出该页面，改动就失效了
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

//用户可调整的设置
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
