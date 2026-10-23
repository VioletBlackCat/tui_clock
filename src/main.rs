mod app;
mod font;
mod ui;
mod theme;
mod config;
mod settings;
mod tools;
//mod 是模块声明，告诉编译器去找同名的.rs文件或同名的目录下的mod.rs文件
//找到了就让编译器编译
use std::io::{self, stdout};
use std::time::Duration;

use crossterm::cursor::{Hide, Show};
use crossterm::event::{self, Event, KeyEventKind};
use crossterm::execute;
use crossterm::terminal::{
    disable_raw_mode, enable_raw_mode, size, EnterAlternateScreen, LeaveAlternateScreen,
};

use app::{App, Screen};
//use导入，要用什么，在这里声明

//fn 名字（参数） -> 返回类型
//Result<T,E> 是标准库的枚举类型，()表示没有意义的返回值，读作 unit（单元类型）
//io::Result<()> 就是 Result<(), io::Error>
//综上，main语义为要么正常结束（返回空值），要么返回io::Error

struct TerminalGuard;
//TerminalGuard是一个空结构体，只有类型，没有字段
//用于在main函数里创建一个变量，利用Rust的所有权和生命周期机制，在main函数结束时自动调用Drop trait的drop方法，恢复终端状态
//即在panic或return时也能保证终端状态恢复，避免终端被卡在原始模式或备用屏幕
//impl Drop trait是Rust标准库提供的一个trait，用于在值离开作用域时执行清理操作
//Drop是必须这样写的，trait契约规定了这个名字
//&mut self是方法接受者，self就是被调用的那个对象
//另：self → 拿走所有权（消费掉）
//&self → 不可变借用
//&mut self → 可变借用（读改写都行，但不夺走所有权）
impl Drop for TerminalGuard {
    fn drop(&mut self) {
        let _ = disable_raw_mode();
        let _ = execute!(stdout(), LeaveAlternateScreen, Show);
    }    
}



fn main() -> io::Result<()> {
    
    let _guard = TerminalGuard;   // 变量以 _ 开头，但不叫 _，所以会活到函数结束

    //仅靠 TerminalGuard 只能恢复终端，但 panic 信息会丢
    //原因：panic 时默认钩子把信息打到 stderr，而此刻还在「备用屏幕」里
    //随后 TerminalGuard::drop 切回主屏幕，备用屏幕连同那条信息一起被丢弃
    //结果就是：终端恢复正常了，但你不知道程序为什么崩
    //修法：先退出备用屏幕，再调用原本的钩子打印信息，让信息留在主屏幕上
    //take_hook() 取出 Rust 默认的钩子（负责打印那条 panic 信息），交给我们保管
    //set_hook() 换成我们自己的，闭包结尾再调用 original_hook(info) 把信息打出来
    //注：set_hook 要求闭包满足 Fn + Send + Sync + 'static，
    //所以必须用 move 把 original_hook（Box<dyn Fn>）的所有权搬进闭包
    //另：本钩子只在 panic = "unwind"（Rust 默认）时有效
    //若 Cargo.toml 里设了 panic = "abort"，栈根本不展开，Drop 和本钩子都不会运行
    let original_hook = std::panic::take_hook();
    std::panic::set_hook(Box::new(move |info| {
        let _ = disable_raw_mode();
        let _ = execute!(stdout(), LeaveAlternateScreen, Show);
        original_hook(info);
    }));

    //let 名字 = 值，如果没有mut，说明名字是不可变的，不能被重新赋值
    //config::load()，调用config.rs模块的函数
    //格式：模块名::函数名
    let settings = config::load();
    
    //首次运行：没有配置文件就生成一份，并把路径告诉用户
    //if let：模式匹配的简写。create_if_missing返回Option<PathBuf>
    //Option是标准库的枚举类型，Some(值)表示有东西，None表示没有东西
    //综上，if let Some(p) = 表达式 { ... }的意思是：如果表达式返回Some(p)，就把p绑定到变量p上，并执行大括号里的代码；
    //如果返回None，就跳过大括号里的代码
    //&p是引用，表示不想拿走p的所有权，只想借用它(另外，由于已经在if let绑定了p，所以p就是settings所返回的东西)

    if let Some(p) = config::create_if_missing(&settings) {
        eprintln!("Default configuration has been generated：{}", p.display());
        eprintln!("（Editable,takes effect on next startup）");
    }
    
    //进入终端原始模式（rawmode），隐藏光标(Hide)，切到备用屏幕（Alternate Screen）
    //退出时要恢复原始模式、显示光标、切回主屏幕
    //另外，退出时要把配置写回文件（失败只提示，不影响退出）
    //“?"运算符：如果返回Ok，就将值取出来继续向下执行；如果返回Err，就结束当前函数，将错误值返回给调用方
    execute!(stdout(), EnterAlternateScreen, Hide)?;
    enable_raw_mode()?;
    //进入原始模式后，Ctrl+C将作为一个普通的按键事件传给程序，而不是直接杀掉程序
    //所以要在程序里处理Ctrl+C，才能让程序退出
    
    //let mut = 可变变量，因为app.handle_key会修改app的状态
    //App::new(settings)是导入app.rs模块的App结构体的关联函数new
    //自此，settings被移动到App里，main函数里不再拥有它的所有权
    let mut app = App::new(settings);
    
    //loop循环，直到用户按下退出键（Esc或Ctrl+C）才会break（退出循环）
    //size()是crossterm的函数，返回终端的宽度和高度
    //用let 将size()的返回值绑定到(term_w, term_h)上，表示终端宽度和高度
    loop {
        let (term_w, term_h) = size()?;

        // 时间驱动的更新（到点响铃）
        if app.update() {
            print!("\x07");          // BEL：终端响铃
        }
        
        ui::draw(&app, term_w, term_h)?;
        //传输的是&app的不可变借用
        //ui负责绘制界面，app负责维护状态
        //在ui使用term_w和term_h时，app的状态不能被修改，所以传递的是不可变借用

        
        //时钟页要每秒重画；菜单/设置页只在有事件时才变；秒表在跑时要高频重画
        //match实际上是表达式，match {...}整体有值
        //Duration是标准库的表达“时长”的类型，from_millis和from_secs是Duration的关联函数，分别表示毫秒和秒
        //_ =>是通配分支符，表示匹配所有其他情况
        //注意顺序：先看当前页面、再看秒表是否在跑
        //若反过来写成 if app.stopwatch.is_running()，秒表在后台跑时会把所有页面
        //都拖进 10Hz 全屏重绘，把之前优化掉的闪烁问题重新放大
        //Screen::Stopwatch if ... 是带守卫的 match 分支：只有「在秒表页」且「在跑」才成立
        let timeout = if app.stopwatch.is_running() || app.timer.is_running() {
            Duration::from_millis(100)
        } else {
            match app.screen {
                Screen::Clock => Duration::from_millis(1000),
                _ => Duration::from_secs(3600),
            }
        };
        //event::poll(timeout)返回Result<bool, Error>，表示是否有事件发生
        //等待timeout时长，用"?"运算符"返回结果，如果有事件发生，就处理事件Ok(true)；
        //如果超时，就继续循环Ok(false)，后面的event::read()就不会执行
        //&& let叫“let链”，允许在if条件中将模式匹配和布尔逻辑结合起来
        //let Event::Key(key)模式匹配event::read()的返回值，如果是键盘事件，就把key绑定到变量key上
        //因为Event有很多变体，例如Key\Mouse\Resize等，所以要用模式匹配来判断是否是键盘事件
        //key.kind == KeyEventKind::Press表示只处理press事件，不处理release事件
        //当事件全部合理时，执行大括号里的代码，调用app.handle_key(key)处理按键事件
        //不用每帧 event::read()（阻塞），是因为这样会导致Clock页的秒针卡住，只有按键时才会刷新
        if event::poll(timeout)?
            && let Event::Key(key) = event::read()?
            && key.kind == KeyEventKind::Press
        {
            app.handle_key(key);
        }
        //app.should_quit是App的一个布尔字段，handle_key 里遇到 Esc→Quit / Ctrl+C 就把它设成 true
        //所以在这里检查它，如果为 true，就退出循环，程序结束(break) 
        if app.should_quit {
            break;
        }
    }
    

    
    //config::save(&app.settings)返回Result<(), Error>，表示保存配置是否成功
    //用if let匹配错误变体 Result<(), Error>，如果成功就什么也不做，如果失败就打印错误信息
    //这个模式叫“吞掉错误但记账”，不用"?"运算符，因为?会在发生错误后直接返回错误并导致程序退出
    if let Err(e) = config::save(&app.settings) {
        eprintln!("Failed to save the configuration：{}", e);
    }

    //Ok()内部是()，表示没有意义的返回值，读作 unit（单元类型），显式表达函数成功返回，没有值
    Ok(())
}

