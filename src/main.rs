use fltk::{
    app,
    button::Button,
    frame::Frame,
    input::Input,
    prelude::*,
    window::Window,
};
use rand::Rng;

fn main() {
    // 1. 初始化应用程序和秘密数字
    let app = app::App::default();
    let secret_number = rand::thread_rng().gen_range(1..=100);

    // 2. 创建主窗口
    let mut win = Window::new(100, 100, 400, 200, "猜数字游戏");

    // 3. 创建UI组件
    let instruction_label = Frame::new(0, 10, 400, 20, "我猜了一个 1-100 之间的数字，是什么？");
    let mut input = Input::new(150, 50, 100, 30, "");
    let mut guess_button = Button::new(150, 90, 100, 30, "猜！");
    let mut result_label = Frame::new(0, 130, 400, 40, "祝你好运！");

    // 4. 设置按钮的点击事件 (核心逻辑)

    // 在闭包外克隆需要在闭包内使用的组件
    // 我们需要修改 input 和 guess_button，所以克隆它们
    let mut input_clone = input.clone();
    let mut button_clone = guess_button.clone();

    guess_button.set_callback(move |_| {
        // 从克隆的 input 获取文本
        let guess_str = input_clone.value();

        // 尝试将文本转换为数字
        match guess_str.trim().parse::<u32>() {
            Ok(num) => {
                if num < secret_number {
                    result_label.set_label("太小了！再试试！");
                } else if num > secret_number {
                    result_label.set_label("太大了！再试试！");
                } else {
                    result_label.set_label("恭喜你，猜对了！");
                    // 使用克隆的组件来禁用它们
                    input_clone.deactivate();
                    button_clone.deactivate();
                }
            },
            Err(_) => {
                result_label.set_label("请输入一个有效的数字！");
            }
        }
    });


    // 5. 结束窗口定义并显示它
    win.end();
    win.show();

    // 6. 运行应用程序，等待用户操作
    app.run().unwrap();
}