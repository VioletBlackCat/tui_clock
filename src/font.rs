pub const GLYPH_WIDTH: usize = 8;
// 字形高度
pub const GLYPH_HEIGHT: usize = 7;
// 字符间距
pub const GLYPH_GAP: usize = 2;
//字符类型
pub type Glyph = [&'static str; GLYPH_HEIGHT];

//修改字符
pub const FILL: char = '▓';

//字符
pub const ZERO: Glyph = [
    "########", 
    "##    ##", 
    "##    ##", 
    "##    ##", 
    "##    ##", 
    "##    ##", 
    "########",
];
pub const ONE: Glyph = [
    "  ###   ", 
    " ####   ", 
    "   ##   ", 
    "   ##   ", 
    "   ##   ", 
    "   ##   ", 
    "########",
];
pub const TWO: Glyph = [
    "########", 
    "      ##", 
    "      ##", 
    "########", 
    "##      ", 
    "##      ", 
    "########",
];
pub const THREE: Glyph = [
    "########", 
    "      ##", 
    "      ##", 
    "########", 
    "      ##", 
    "      ##", 
    "########",
];
pub const FOUR: Glyph = [
    "##    ##", 
    "##    ##", 
    "##    ##", 
    "##    ##", 
    "########", 
    "      ##", 
    "      ##",
];
pub const FIVE: Glyph = [
    "########", 
    "##      ", 
    "##      ", 
    "########",     
    "      ##", 
    "      ##", 
    "########",
];
pub const SIX: Glyph = [
    "########", 
    "##      ", 
    "##      ", 
    "########", 
    "##    ##", 
    "##    ##", 
    "########",
];
pub const SEVEN: Glyph = [
    "########", 
    "      ##", 
    "    ##  ", 
    "   ##   ", 
    "   ##   ", 
    "   ##   ", 
    "   ##   ",
];
pub const EIGHT: Glyph = [
    "########", 
    "##    ##", 
    "##    ##", 
    "########", 
    "##    ##", 
    "##    ##", 
    "########",
];
pub const NINE: Glyph = [
    "########", 
    "##    ##", 
    "##    ##", 
    "########", 
    "      ##", 
    "      ##", 
    "########",
];
pub const COLON: Glyph = [
    "        ", 
    "   ##   ", 
    "   ##   ", 
    "        ", 
    "   ##   ", 
    "   ##   ", 
    "        ",
];
pub const COLON_OFF:Glyph = [
    "        ", 
    "        ", 
    "        ", 
    "        ", 
    "        ", 
    "        ", 
    "        ",
];

pub fn glyph_for(c: char, colon_on:bool) -> Option<&'static Glyph> {
    match c {
        '0' => Some(&ZERO),
        '1' => Some(&ONE),
        '2' => Some(&TWO),
        '3' => Some(&THREE),
        '4' => Some(&FOUR),
        '5' => Some(&FIVE),
        '6' => Some(&SIX),
        '7' => Some(&SEVEN),
        '8' => Some(&EIGHT),
        '9' => Some(&NINE),
        ':' => Some(if colon_on { &COLON } else { &COLON_OFF }),
        _ => None,
    }
}
