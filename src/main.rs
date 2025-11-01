use fltk::{
    app,
    button::Button,
    frame::Frame,
    input::Input,
    menu::Choice, // <--- 已修正此处的导入路径
    prelude::*,
    text::{TextBuffer, TextDisplay}, // 用于历史记录
    window::Window,
    group::Pack, // 用于UI布局
};
use rand::Rng;
use std::ops::RangeInclusive; // 引入范围类型

// ========= 1. 常量与配置 =========

const WINDOW_WIDTH: i32 = 480;
const WINDOW_HEIGHT: i32 = 600;

// ========= 2. 定义游戏核心数据结构 =========

/// 游戏难度枚举
#[derive(Debug, Clone, Copy, PartialEq)]
enum Difficulty {
    Easy,
    Medium,
    Hard,
}

impl Difficulty {
    /// 根据难度获取数字范围
    fn range(&self) -> RangeInclusive<u32> {
        match self {
            Difficulty::Easy => 1..=100,
            Difficulty::Medium => 1..=500,
            Difficulty::Hard => 1..=1000,
        }
    }

    /// 根据难度获取最大尝试次数
    fn attempts(&self) -> u32 {
        match self {
            Difficulty::Easy => 10,
            Difficulty::Medium => 9,
            Difficulty::Hard => 8,
        }
    }
}

/// 统一管理游戏状态
struct GameState {
    secret_number: u32,
    remaining_attempts: u32,
    difficulty: Difficulty,
    game_over: bool,
}

impl GameState {
    /// 创建一个新游戏的状态
    fn new(difficulty: Difficulty) -> Self {
        let secret_number = rand::thread_rng().gen_range(difficulty.range());
        println!("新游戏开始！难度: {:?}, 秘密数字是: {}", difficulty, secret_number); // 在控制台打印答案，方便调试
        Self {
            secret_number,
            remaining_attempts: difficulty.attempts(),
            difficulty,
            game_over: false,
        }
    }
}

/// 定义UI和游戏逻辑之间的通信消息
#[derive(Clone, Copy)]
enum GameMessage {
    Guess,
    NewGame,
}


// ========= 3. 主函数：应用程序入口 =========

fn main() {
    // 1. 初始化应用程序和消息通道
    let app = app::App::default().with_scheme(app::Scheme::Gtk);
    let (sender, receiver) = app::channel::<GameMessage>();

    // 2. 初始化游戏状态
    let mut game_state = GameState::new(Difficulty::Easy);

    // 3. 创建主窗口和UI布局容器
    let mut main_window = Window::new(100, 100, WINDOW_WIDTH, WINDOW_HEIGHT, "高级猜数字游戏");
    let mut main_pack = Pack::new(10, 10, WINDOW_WIDTH - 20, WINDOW_HEIGHT - 20, "");
    main_pack.set_spacing(15); // 设置组件间距

    // --- 顶部UI：游戏说明 ---
    let _instruction_label = Frame::default()
        .with_size(0, 40)
        .with_label("猜一个秘密数字！选择难度并开始游戏。");

    // --- 难度选择区域 ---
    let mut controls_pack = Pack::new(0, 0, 0, 30, "");
    controls_pack.set_type(fltk::group::PackType::Horizontal); // 水平布局
    controls_pack.set_spacing(10);
    let _difficulty_label = Frame::default().with_size(120, 0).with_label("选择难度:");
    let mut difficulty_chooser = Choice::default().with_size(150, 0);
    difficulty_chooser.add_choice("简单 (1-100)");
    difficulty_chooser.add_choice("中等 (1-500)");
    difficulty_chooser.add_choice("困难 (1-1000)");
    difficulty_chooser.set_value(0); // 默认选择“简单”
    let mut new_game_button = Button::default().with_size(120, 0).with_label("新游戏");
    controls_pack.end();

    // --- 输入与猜测区域 ---
    let mut guess_pack = Pack::new(0, 0, 0, 30, "");
    guess_pack.set_type(fltk::group::PackType::Horizontal);
    guess_pack.set_spacing(10);
    let mut guess_input = Input::default().with_size(250, 0);
    guess_input.set_tooltip("在这里输入你猜的数字");
    let mut guess_button = Button::default().with_size(150, 0).with_label("猜！");
    guess_pack.end();
    
    // --- 状态与反馈信息 ---
    let mut status_label = Frame::default()
        .with_size(0, 20)
        .with_label(&format!("剩余尝试次数: {}", game_state.remaining_attempts));
    let mut result_label = Frame::default()
        .with_size(0, 40)
        .with_label("祝你好运！");
    result_label.set_label_size(20);

    // --- 猜测历史记录 ---
    let _history_label = Frame::default().with_size(0, 20).with_label("猜测历史:");
    let mut history_buffer = TextBuffer::default();
    let mut history_display = TextDisplay::new(0, 0, 0, 150, "");
    history_display.set_buffer(history_buffer.clone());


    main_pack.end();
    main_window.end();
    main_window.show();


    // 4. 设置按钮回调，通过发送消息来触发逻辑
    guess_button.emit(sender, GameMessage::Guess);
    new_game_button.emit(sender, GameMessage::NewGame);


    // 5. 主事件循环：监听并处理消息
    while app.wait() {
        if let Some(msg) = receiver.recv() {
            match msg {
                // --- 处理“新游戏”消息 ---
                GameMessage::NewGame => {
                    let selected_difficulty = match difficulty_chooser.value() {
                        1 => Difficulty::Medium,
                        2 => Difficulty::Hard,
                        _ => Difficulty::Easy,
                    };
                    game_state = GameState::new(selected_difficulty);

                    // 重置UI
                    status_label.set_label(&format!("剩余尝试次数: {}", game_state.remaining_attempts));
                    result_label.set_label("新游戏开始，祝你好运！");
                    result_label.set_label_color(fltk::enums::Color::Black);
                    guess_input.activate();
                    guess_button.activate();
                    guess_input.set_value("");
                    history_buffer.set_text("");
                },

                // --- 处理“猜测”消息 ---
                GameMessage::Guess => {
                    if game_state.game_over { continue; } // 如果游戏已结束，则不处理

                    let guess_str = guess_input.value();
                    guess_input.set_value(""); // 清空输入框

                    match guess_str.trim().parse::<u32>() {
                        Ok(num) => {
                            game_state.remaining_attempts -= 1;
                            let current_history = history_buffer.text();
                            
                            if num < game_state.secret_number {
                                result_label.set_label("太小了！再试试！");
                                result_label.set_label_color(fltk::enums::Color::from_rgb(200, 100, 0)); // 橙色
                                history_buffer.set_text(&format!("{} > {} (太小了)\n", current_history, num));

                            } else if num > game_state.secret_number {
                                result_label.set_label("太大了！再试试！");
                                result_label.set_label_color(fltk::enums::Color::from_rgb(200, 100, 0)); // 橙色
                                history_buffer.set_text(&format!("{} > {} (太大了)\n", current_history, num));

                            } else {
                                result_label.set_label("恭喜你，猜对了！");
                                result_label.set_label_color(fltk::enums::Color::from_rgb(0, 150, 0)); // 绿色
                                history_buffer.set_text(&format!("{} > {} (正确!)\n", current_history, num));
                                game_state.game_over = true;
                                guess_input.deactivate();
                                guess_button.deactivate();
                            }

                            // 检查是否次数用尽
                            if !game_state.game_over && game_state.remaining_attempts == 0 {
                                result_label.set_label(&format!("游戏结束！答案是: {}", game_state.secret_number));
                                result_label.set_label_color(fltk::enums::Color::from_rgb(200, 0, 0)); // 红色
                                game_state.game_over = true;
                                guess_input.deactivate();
                                guess_button.deactivate();
                            }

                            status_label.set_label(&format!("剩余尝试次数: {}", game_state.remaining_attempts));

                        },
                        Err(_) => {
                            result_label.set_label("请输入一个有效的正整数！");
                            result_label.set_label_color(fltk::enums::Color::Red);
                        }
                    }
                },
            }
        }
    }
}