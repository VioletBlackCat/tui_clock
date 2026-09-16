use crossterm::style::Color;


// 一套配色方案。Copy的 supertrait是Clone,两者必须一起 derive
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Theme {
    Classic,
    Sunset,
    Ocean,
}
//显示在设置中的文字
impl Theme {
     //全部主题,顺序与settings 里的主题顺序相同
     pub const ALL: [Theme; 3] = [Theme::Classic, Theme::Sunset, Theme::Ocean];
    // 从配置里的名字还原主题（大小写不敏感）
     pub fn from_name(s: &str) -> Option<Self> {
        Self::ALL.iter().copied().find(|t| t.label().eq_ignore_ascii_case(s))
     }
     
     
     pub fn label(self) -> &'static str {
        match self {
            Theme::Classic => "Classic",
            Theme::Ocean => "Ocean",
            Theme::Sunset => "Sunset"
        }
     }

    //切到下一套主题，循环
    pub fn next(self) -> Self {
        match self {
            Theme::Classic => Theme::Sunset,
            Theme::Sunset => Theme::Ocean,
            Theme::Ocean => Theme::Classic,
        }
    }
    // 取 t ∈ [0,1] 处的颜色：0 = 顶，1 = 底
    pub fn color_at(self, t: f32) -> Color {
        let (top, bottom) = self.stops();
        lerp_rgb(top, bottom, t)
    }
    // 色标：(顶色, 底色)
    fn stops(self) -> ((u8, u8, u8), (u8, u8, u8)) {
        match self {
            Theme::Classic => ((240, 240, 240), (240, 240, 240)), // 纯色，无渐变
            Theme::Sunset  => ((255, 120, 60), (170, 50, 180)),   // 橙 → 紫
            Theme::Ocean   => ((0, 220, 200), (40, 90, 255)),     // 青 → 蓝
        }
    }
}

// 把 (r,g,b) 元组包成crossterm认识的的颜色
fn rgb(c: (u8, u8, u8)) -> Color {
    Color::Rgb { r: c.0, g: c.1, b: c.2 }
}
// 单个通道插值：t=0 → a，t=1 → b（全程 f32，避免u8下溢）
fn lerp_u8(a: u8, b: u8, t: f32) -> u8 {
    let (a, b) = (a as f32, b as f32);
    (a + (b - a) * t).round().clamp(0.0, 255.0) as u8
}

fn lerp_rgb(from: (u8, u8, u8), to: (u8, u8, u8), t: f32) -> Color {
    rgb((
        lerp_u8(from.0, to.0, t),
        lerp_u8(from.1, to.1, t),
        lerp_u8(from.2, to.2, t),
    ))
}



