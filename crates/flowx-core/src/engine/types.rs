use serde::{Deserialize, Serialize};
use thiserror::Error;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Point {
    pub x: i32,
    pub y: i32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Rect {
    pub x: i32,
    pub y: i32,
    pub width: u32,
    pub height: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Color {
    pub r: u8,
    pub g: u8,
    pub b: u8,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Direction {
    Up,
    Down,
    Left,
    Right,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Key {
    Back,
    Home,
    Menu,
    Enter,
    Delete,
    VolumeUp,
    VolumeDown,
    Down,
    Up,
    Left,
    Right,
    Other(String),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Element {
    pub id: String,
    pub text: Option<String>,
    pub bounds: Rect,
    pub class_name: Option<String>,
    pub clickable: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Image {
    pub data: Vec<u8>,
    pub width: u32,
    pub height: u32,
    pub format: ImageFormat,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ImageFormat {
    RGB,
    RGBA,
    Gray,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Text {
    pub content: String,
    pub bounds: Rect,
    pub confidence: f32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum CommandResult {
    Success,
    Element(Element),
    Elements(Vec<Element>),
    Image(Image),
    Text(String),
    Texts(Vec<Text>),
    Boolean(bool),
    Number(i32),
    Size(u32, u32),
}

#[derive(Debug, Error)]
pub enum CommandError {
    #[error("Element not found: {selector:?}")]
    ElementNotFound { selector: String },

    #[error("Operation timed out: {operation} after {timeout_ms}ms")]
    Timeout { operation: String, timeout_ms: u64 },

    #[error("Platform error: {message}")]
    PlatformError { message: String },

    #[error("Permission denied: {permission}")]
    PermissionDenied { permission: String },

    #[error("Invalid argument: {param} - {reason}")]
    InvalidArgument { param: String, reason: String },
}
