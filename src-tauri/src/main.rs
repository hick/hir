// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use tauri::{Manager, GlobalShortcutManager};
use tauri::{SystemTray, SystemTrayMenu, SystemTrayMenuItem, CustomMenuItem, SystemTrayEvent};
use tauri::{Menu, MenuItem, Submenu};

// use tauri::{CustomMenuItem, SystemTray, SystemTrayMenu, SystemTrayMenuItem};


// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

fn main() {
    // 创建系统托盘的对象和菜单
    let quit = CustomMenuItem::new("quit".to_string(), "Quit");
    let hide = CustomMenuItem::new("hide".to_string(), "Hide");
    let tray_menu = SystemTrayMenu::new()
        .add_item(quit)
        .add_native_item(SystemTrayMenuItem::Separator)
        .add_item(hide); // insert the menu items here
    let system_tray = SystemTray::new()
      .with_menu(tray_menu);

    // 创建正常菜单, 参考官方 https://tauri.app/v1/guides/features/menu
    let quit2 = CustomMenuItem::new("quit2".to_string(), "Quit");
    let close2 = CustomMenuItem::new("close2".to_string(), "Close");
    let submenu = Submenu::new("File", Menu::new().add_item(quit2).add_item(close2));
    let menu = Menu::new()
      .add_native_item(MenuItem::Copy)
      .add_item(CustomMenuItem::new("hide2", "Hide"))
      .add_submenu(submenu);
    
    // tauri::Builder::default() 创建一个默认设置的 Tauri 应用构建器
    tauri::Builder::default()
        // 普通菜单: mac 下是左上角的菜单栏
        .menu(menu)
        // .添加系统托盘的动作
        .system_tray(system_tray)
        // 系统菜单的事件
        .on_system_tray_event(|app, event| match event {
            SystemTrayEvent::LeftClick {
              position: _,
              size: _,
              ..
            } => {
              println!("system tray received a left click");
            }
            SystemTrayEvent::RightClick {
              position: _,
              size: _,
              ..
            } => {
              println!("system tray received a right click");
            }
            SystemTrayEvent::DoubleClick {
              position: _,
              size: _,
              ..
            } => {
              println!("system tray received a double click");
            }
            SystemTrayEvent::MenuItemClick { id, .. } => {
              match id.as_str() {
                "quit" => {
                  std::process::exit(0);
                }
                "hide" => {
                  let window = app.get_window("main").unwrap();
                  window.hide().unwrap();
                }
                _ => {}
              }
            }
            _ => {}
        })
        // .setup 是 应用程序初始化时执行自定义代码
        // 接受一个闭包（一个匿名函数），该闭包的参数是 App 类型的实例，代表当前的 Tauri 应用
        .setup(|app| {
            // 获取名为 "main" 的窗口。这个名字通常是在 tauri.conf.json 配置文件中定义的窗口标识符(2024-01-27 实际没发现, 可能是默认的名字)
            let window = app.get_window("main").unwrap();
            // 设置不显示关闭按钮等: 实际整个标题栏都没了: true 改成 false 就不会显示了
            window.set_decorations(true).expect("Failed to set window decorations");
            // 2024-01-27 从官方文档确认最新的肯定是有的, 我现在老版本应该是没有 
            // https://docs.rs/tauri/1.5.4/tauri/window/struct.Window.html#method.set_closable
            // 2024-01-27 新版本支持的禁用关闭按钮的方式
            window.set_closable(false).expect("Failed to set window closable");
            // 设置置顶
            window.set_always_on_top(true).unwrap();

            // 创建新窗口
            // let docs_window = tauri::WindowBuilder::new(
            let editor_window = tauri::WindowBuilder::new(
              app,
              "external", /* the unique window label */
              tauri::WindowUrl::External("https://slyt8.cn/hms/contact/action?actionId=100&search=&op=info&infoid=".parse().unwrap())
            ).build()?;
            editor_window.set_closable(false).unwrap();
            editor_window.set_always_on_top(true).unwrap();
            let size: tauri::PhysicalSize<u32> = editor_window.inner_size().unwrap();
            let new_width = (size.width as f64 * 0.8) as f64;
            let new_height = (size.height as f64 * 0.5) as f64;
            // editor_window.set_position(new_width, new_height).unwrap();
            // let new_width = (size.width as f64 * scale) as f64;
            // let new_height = (size.height as f64 * scale) as f64;
            // // 调整窗口大小
            editor_window.set_size(tauri::PhysicalSize { width: new_width, height: new_height }).unwrap();
            // === 调整位置
            // 获取当前窗口的位置
            let position = editor_window.outer_position().unwrap();
            // 计算新的位置
            let new_x = position.x - 100;
            let new_y = position.y + 100;
            // 设置窗口位置
            editor_window.set_position(tauri::PhysicalPosition { x: new_x, y: new_y }).unwrap();


            // 240127-041259 注册全局快捷键
            app.global_shortcut_manager()
                .register("Ctrl+Alt+2", move || {
                    let is_visible = window.is_visible().unwrap();
                    if is_visible {
                        window.hide().unwrap();
                    } else {
                        window.show().unwrap();
                        window.set_focus().unwrap();
                    }
                })
                .expect("Failed to register global shortcut");
            Ok(())
        })
        // 定义了一个处理从 JavaScript 发送到 Rust 的消息的回调函数。在这个例子中，它注册了一个名为 “greet” 的命令。
        .invoke_handler(tauri::generate_handler![greet])
        // 这个函数启动 Tauri 应用程序。它使用 tauri::generate_context!() 宏来生成应用程序需要的上下文。
        .run(tauri::generate_context!())
        // 这个函数是 Rust 的 Result 类型的方法，用于处理可能的错误。如果 run 函数返回一个错误，它会停止程序的执行并打印一条错误消息。
        .expect("error while running tauri application");
}