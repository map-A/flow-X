use clap::{Parser, Subcommand};
use flowx_core::engine::{AsyncCommandExecutor, Command, Point};

use anyhow::Result;
#[derive(Parser)]
#[command(name = "flowx")]
#[command(about = "FlowX CLI - 跨平台自动化工具")]
struct Cli {
    #[arg(short, long, help = "设备地址 (例如: android://localhost:6789, macos)")]
    device: String,

    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    #[command(about = "点击屏幕")]
    Click {
        #[arg(help = "X 坐标")]
        x: i32,
        #[arg(help = "Y 坐标")]
        y: i32,
    },
    #[command(about = "滑动手势")]
    Swipe {
        #[arg(help = "起点 X")]
        x1: i32,
        #[arg(help = "起点 Y")]
        y1: i32,
        #[arg(help = "终点 X")]
        x2: i32,
        #[arg(help = "终点 Y")]
        y2: i32,
        #[arg(short, long, default_value = "300", help = "持续时间(ms)")]
        duration: u64,
    },
    #[command(about = "输入文本")]
    Input {
        #[arg(help = "要输入的文本")]
        text: String,
    },
    #[command(about = "按键操作")]
    Key {
        #[arg(help = "按键名称 (BACK, HOME, MENU 等)")]
        key: String,
    },
    #[command(about = "查找元素")]
    Find {
        #[arg(help = "要查找的文本")]
        text: String,
    },
    #[command(about = "获取设备信息")]
    Info,
    #[command(about = "截图")]
    Screenshot {
        #[arg(short, long, help = "输出文件路径")]
        output: Option<String>,
    },
}

#[tokio::main]
async fn main() -> Result<()> {
    let cli = Cli::parse();

    println!("FlowX CLI v0.1.0");
    println!("连接设备: {}", cli.device);
    println!();

    // 创建统一的执行器
    let executor: Box<dyn AsyncCommandExecutor> =
        if cli.device.starts_with("android://") || cli.device.contains(':') {
            Box::new(flowx_core::platforms::AndroidClient::new(cli.device))
        } else if cli.device == "macos" {
            #[cfg(target_os = "macos")]
            {
                Box::new(flowx_core::platforms::MacOSClient::new()?)
            }
            #[cfg(not(target_os = "macos"))]
            {
                anyhow::bail!("macOS 平台仅在 macOS 上可用");
            }
        } else if cli.device == "windows" {
            #[cfg(target_os = "windows")]
            {
                Box::new(flowx_core::platforms::WindowsClient::new())
            }
            #[cfg(not(target_os = "windows"))]
            {
                anyhow::bail!("Windows 平台仅在 Windows 上可用");
            }
        } else {
            // 默认作为 Android 设备
            Box::new(flowx_core::platforms::AndroidClient::new(format!(
                "android://{}",
                cli.device
            )))
        };

    match cli.command {
        Commands::Click { x, y } => {
            println!("执行: 点击 ({}, {})", x, y);
            let result = executor.execute_async(Command::Click { x, y }).await?;
            println!("✅ 点击成功: {:?}", result);
        }
        Commands::Swipe {
            x1,
            y1,
            x2,
            y2,
            duration,
        } => {
            println!(
                "执行: 滑动 ({},{}) → ({},{}) [{}ms]",
                x1, y1, x2, y2, duration
            );
            let result = executor
                .execute_async(Command::Swipe {
                    from: Point { x: x1, y: y1 },
                    to: Point { x: x2, y: y2 },
                    duration_ms: duration,
                })
                .await?;
            println!("✅ 滑动成功: {:?}", result);
        }
        Commands::Input { text } => {
            println!("执行: 输入文本 \"{}\"", text);
            let result = executor.execute_async(Command::InputText { text }).await?;
            println!("✅ 输入成功: {:?}", result);
        }
        Commands::Key { key } => {
            println!("执行: 按键 {}", key);
            let key_enum = parse_key(&key);
            let result = executor
                .execute_async(Command::PressKey { key: key_enum })
                .await?;
            println!("✅ 按键成功: {:?}", result);
        }
        Commands::Find { text } => {
            println!("执行: 查找元素 \"{}\"", text);
            let result = executor
                .execute_async(Command::FindElement {
                    selector: flowx_core::engine::Selector::Text(text.clone()),
                })
                .await?;
            println!("✅ 找到元素: {:?}", result);
        }
        Commands::Info => {
            println!("执行: 获取设备信息");
            let result = executor.execute_async(Command::GetScreenSize).await?;
            if let flowx_core::engine::CommandResult::Size(w, h) = result {
                println!("✅ 屏幕尺寸: {}x{}", w, h);
            }
        }
        Commands::Screenshot { output } => {
            println!("执行: 截图");
            let result = executor
                .execute_async(Command::Screenshot { region: None })
                .await?;

            if let flowx_core::engine::CommandResult::Image(image) = result {
                if let Some(path) = output {
                    std::fs::write(&path, &image.data)?;
                    println!(
                        "✅ 截图已保存: {} ({}x{}, {} bytes)",
                        path,
                        image.width,
                        image.height,
                        image.data.len()
                    );
                } else {
                    println!(
                        "✅ 截图完成: {}x{} ({} bytes)",
                        image.width,
                        image.height,
                        image.data.len()
                    );
                    println!("提示: 使用 --output <文件路径> 保存截图");
                }
            } else {
                println!("⚠️  截图返回了意外的结果");
            }
        }
    }

    Ok(())
}

fn parse_key(key_str: &str) -> flowx_core::engine::Key {
    match key_str.to_uppercase().as_str() {
        "BACK" => flowx_core::engine::Key::Back,
        "HOME" => flowx_core::engine::Key::Home,
        "MENU" => flowx_core::engine::Key::Menu,
        "ENTER" => flowx_core::engine::Key::Enter,
        "DELETE" | "DEL" => flowx_core::engine::Key::Delete,
        "VOLUME_UP" => flowx_core::engine::Key::VolumeUp,
        "VOLUME_DOWN" => flowx_core::engine::Key::VolumeDown,
        "UP" => flowx_core::engine::Key::Up,
        "DOWN" => flowx_core::engine::Key::Down,
        "LEFT" => flowx_core::engine::Key::Left,
        "RIGHT" => flowx_core::engine::Key::Right,
        _ => flowx_core::engine::Key::Other(key_str.to_string()),
    }
}
