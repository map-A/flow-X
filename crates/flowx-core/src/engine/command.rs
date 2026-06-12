use super::types::{Color, Direction, Image, Key, Point, Rect};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Command {
    // 触摸操作
    Click {
        x: i32,
        y: i32,
    },
    LongClick {
        x: i32,
        y: i32,
        duration_ms: u64,
    },
    Swipe {
        from: Point,
        to: Point,
        duration_ms: u64,
    },
    Drag {
        from: Point,
        to: Point,
    },

    // 输入操作
    InputText {
        text: String,
    },
    PressKey {
        key: Key,
    },

    // 元素查找
    FindElement {
        selector: Selector,
    },
    FindElements {
        selector: Selector,
    },
    WaitForElement {
        selector: Selector,
        timeout_ms: u64,
    },

    // 屏幕操作
    Screenshot {
        region: Option<Rect>,
    },
    GetScreenSize,
    ScrollTo {
        direction: Direction,
        distance: i32,
    },

    // 应用管理
    OpenApp {
        name: String,
    },
    CloseApp {
        package: String,
    },
    GetCurrentApp,

    // 视觉识别
    OCR {
        region: Option<Rect>,
    },
    FindImage {
        template: Image,
        threshold: f32,
    },
    FindColor {
        color: Color,
        region: Option<Rect>,
    },
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Selector {
    Text(String),
    TextContains(String),
    TextMatches(String), // Regex pattern as string
    Id(String),
    ClassName(String),
    XPath(String),
    Image { template: Vec<u8>, threshold: f32 },
    And(Box<Selector>, Box<Selector>),
    Or(Box<Selector>, Box<Selector>),
    Parent(Box<Selector>),
    Child(Box<Selector>, usize),
}
