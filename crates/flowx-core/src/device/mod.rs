/// Device control module
///
/// Handles device-level operations like screen control, input injection, and app management
use crate::engine::{
    AsyncCommandExecutor, Command, CommandError, CommandResult, Element, Image, Point, Rect,
    Selector, Text,
};

/// Device controller trait for platform-specific device operations
pub trait DeviceController: Send + Sync {
    /// Get screen size
    fn get_screen_size(&self) -> Result<(u32, u32), CommandError>;

    /// Inject touch event at specific coordinates
    fn tap(&self, point: Point) -> Result<(), CommandError>;

    /// Inject long press event
    fn long_press(&self, point: Point, duration_ms: u64) -> Result<(), CommandError>;

    /// Inject swipe gesture
    fn swipe(&self, from: Point, to: Point, duration_ms: u64) -> Result<(), CommandError>;
}

/// High-level Device API that wraps the command executor
pub struct Device<E: AsyncCommandExecutor> {
    executor: E,
}

impl<E: AsyncCommandExecutor> Device<E> {
    pub fn new(executor: E) -> Self {
        Self { executor }
    }

    /// Click at specific coordinates
    pub async fn click(&self, x: i32, y: i32) -> Result<(), CommandError> {
        self.executor.execute_async(Command::Click { x, y }).await?;
        Ok(())
    }

    /// Long click at specific coordinates
    pub async fn long_click(&self, x: i32, y: i32, duration_ms: u64) -> Result<(), CommandError> {
        self.executor
            .execute_async(Command::LongClick { x, y, duration_ms })
            .await?;
        Ok(())
    }

    /// Swipe from one point to another
    pub async fn swipe(
        &self,
        x1: i32,
        y1: i32,
        x2: i32,
        y2: i32,
        duration_ms: u64,
    ) -> Result<(), CommandError> {
        self.executor
            .execute_async(Command::Swipe {
                from: Point { x: x1, y: y1 },
                to: Point { x: x2, y: y2 },
                duration_ms,
            })
            .await?;
        Ok(())
    }

    /// Input text
    pub async fn input_text(&self, text: &str) -> Result<(), CommandError> {
        self.executor
            .execute_async(Command::InputText {
                text: text.to_string(),
            })
            .await?;
        Ok(())
    }

    /// Find a single element by selector
    pub async fn find_element(&self, selector: Selector) -> Result<Element, CommandError> {
        match self
            .executor
            .execute_async(Command::FindElement { selector })
            .await?
        {
            CommandResult::Element(element) => Ok(element),
            _ => Err(CommandError::PlatformError {
                message: "Unexpected result type".to_string(),
            }),
        }
    }

    /// Find all elements matching selector
    pub async fn find_elements(&self, selector: Selector) -> Result<Vec<Element>, CommandError> {
        match self
            .executor
            .execute_async(Command::FindElements { selector })
            .await?
        {
            CommandResult::Elements(elements) => Ok(elements),
            _ => Err(CommandError::PlatformError {
                message: "Unexpected result type".to_string(),
            }),
        }
    }

    /// Take a screenshot
    pub async fn screenshot(&self, region: Option<Rect>) -> Result<Image, CommandError> {
        match self
            .executor
            .execute_async(Command::Screenshot { region })
            .await?
        {
            CommandResult::Image(image) => Ok(image),
            _ => Err(CommandError::PlatformError {
                message: "Unexpected result type".to_string(),
            }),
        }
    }

    /// Get screen size
    pub async fn get_screen_size(&self) -> Result<(u32, u32), CommandError> {
        match self.executor.execute_async(Command::GetScreenSize).await? {
            CommandResult::Size(width, height) => Ok((width, height)),
            _ => Err(CommandError::PlatformError {
                message: "Unexpected result type".to_string(),
            }),
        }
    }

    /// Open an app by name
    pub async fn open_app(&self, name: &str) -> Result<(), CommandError> {
        self.executor
            .execute_async(Command::OpenApp {
                name: name.to_string(),
            })
            .await?;
        Ok(())
    }

    /// Press a key
    pub async fn press_key(&self, key: &str) -> Result<(), CommandError> {
        use crate::engine::Key;
        let key_enum = match key {
            "Enter" => Key::Enter,
            "Down" => Key::Down,
            "CommandT" => Key::Other(key.to_string()),
            _ => Key::Other(key.to_string()),
        };
        self.executor
            .execute_async(Command::PressKey { key: key_enum })
            .await?;
        Ok(())
    }

    /// Perform OCR on screen region
    pub async fn ocr(&self, region: Option<Rect>) -> Result<Vec<Text>, CommandError> {
        match self.executor.execute_async(Command::OCR { region }).await? {
            CommandResult::Texts(texts) => Ok(texts),
            _ => Err(CommandError::PlatformError {
                message: "Unexpected result type".to_string(),
            }),
        }
    }
}
