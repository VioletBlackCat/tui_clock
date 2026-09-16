use std::path::PathBuf;
use std::fs;
use std::io;

use crate::app::{HourFormat, Settings};
use crate::theme::Theme;


/// 首次运行：配置文件不存在就生成一份，返回「刚创建的路径」（已存在则返回 None）
pub fn create_if_missing(settings: &Settings) -> Option<PathBuf> {
    let path = config_path()?;      // 拿不到路径 → 提前返回 None
    if path.exists() {
        return None;                // 已经有了，什么都不做
    }

    match save(settings) {
        Ok(()) => Some(path),       // 生成成功 → 把路径交出去，让调用方去提示
        Err(e) => {
            eprintln!("生成默认配置失败：{}", e);
            None
        }
    }
}



// 读取配置。任何异常都降级成默认值，保证程序一定能起来。
pub fn load() -> Settings {
    let Some(path) = config_path() else {
        return Settings::default();     // 连路径都算不出来,环境变量缺失
    };

    match fs::read_to_string(&path) {
        Ok(text) => parse(&text),
        //文件不存在 = 首次运行，这是正常路径，不是错误
        Err(e) if e.kind() == io::ErrorKind::NotFound => Settings::default(),
        //遇到其他错误时使用默认配置
        Err(e) => {
            eprintln!("Falied to load configuration（{}）", e);
            Settings::default()
        }
    }
}
// 保存配置。失败可以上报，由调用方决定怎么办
pub fn save(s: &Settings) -> io::Result<()> {
    let Some(path) = config_path() else {
        return Ok(())
    };
    // 目录可能不存在,建出来
    if let Some(dir) = path.parent() {
        fs::create_dir_all(dir)?;        
    }

    fs::write(&path, serialize(s))
}


/// 把配置文本解析成 Settings。
/// 宽容策略：认不出的行/键/值一律跳过，保留默认值。
pub fn parse(text: &str) -> Settings {
    let mut settings = Settings::default();

    for line in text.lines() {
        let line = line.trim();

        // 空行、注释（# 开头）跳过
        if line.is_empty() || line.starts_with('#') {
            continue;
        }

        //key = value
        let Some((key, value)) = line.split_once('=') else {
            continue; // 没有等号的行:忽略
        };
        let key = key.trim();
        let value = value.trim().trim_matches('"'); // 容忍 "24H" 和 24H 两种写法

        match key {
            "hour_format" => {
                if let Some(f) = HourFormat::from_name(value) {
                    settings.hour_format = f;
                }
            }
            "show_seconds" => {
                if let Some(b) = parse_bool(value) {
                    settings.show_seconds = b;
                }
            }
            "theme" => {
                if let Some(t) = Theme::from_name(value) {
                    settings.theme = t;
                }
            }
            _ => {} // 未知的键：忽略（向前兼容）
        }
    }

    settings
}

/// true / false，大小写不敏感
fn parse_bool(s: &str) -> Option<bool> {
    if s.eq_ignore_ascii_case("true") {
        Some(true)
    } else if s.eq_ignore_ascii_case("false") {
        Some(false)
    } else {
        None
    }
}

/// 把设置写成配置文本（带注释，方便用户直接编辑）
pub fn serialize(s: &Settings) -> String {
    let themes: Vec<&str> = Theme::ALL.iter().map(|t| t.label()).collect();
    let hours: Vec<&str> = HourFormat::ALL.iter().map(|f| f.label()).collect();

    format!(
        "# tui_clock 配置文件 —— 可自由编辑，保存后下次启动生效\n\
         #\n\
         # hour_format  : {}\n\
         # show_seconds : true | false\n\
         # theme        : {}\n\
         \n\
         hour_format  = \"{}\"\n\
         show_seconds = {}\n\
         theme        = \"{}\"\n",
        hours.join(" | "),
        themes.join(" | "),
        s.hour_format.label(),
        s.show_seconds,
        s.theme.label(),
    )
}

//系统约定的用户配置目录
fn config_dir() -> Option<PathBuf> {
    if cfg!(windows) {
        std::env::var_os("APPDATA").
        map(PathBuf::from)
    } 
    else if cfg!(target_os = "macos") {
        std::env::var_os("HOME")
        .map(|h| PathBuf::from(h).join("Library/Application Support"))
    } 
    else {
        std::env::var_os("XDG_CONFIG_HOME")
        .map(PathBuf::from)
        .or_else(|| std::env::var_os("HOME").map(|h| PathBuf::from(h).join(".config")))
    }
}

//本应用的完整配置名
pub fn config_path() -> Option<PathBuf> {
    config_dir().map(|d| d.join("tui_clock").join("config.toml"))
}