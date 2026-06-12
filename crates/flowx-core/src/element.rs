use crate::engine::{Command, CommandError, Point, Selector};

/// 元素查找器
pub struct ElementFinder;

impl ElementFinder {
    pub fn new() -> Self {
        Self
    }

    /// 查找单个元素
    pub fn find(&self, selector: &Selector) -> Result<Option<Element>, CommandError> {
        match selector {
            Selector::Text(text) => self.find_by_text(text),
            Selector::TextContains(text) => self.find_by_text_contains(text),
            Selector::Id(id) => self.find_by_id(id),
            Selector::ClassName(class) => self.find_by_class(class),
            _ => Err(CommandError::InvalidArgument {
                param: "selector".to_string(),
                reason: "Unsupported selector type".to_string(),
            }),
        }
    }

    fn find_by_text(&self, text: &str) -> Result<Option<Element>, CommandError> {
        // 平台实现将覆盖此方法
        Ok(Some(Element {
            id: "mock".to_string(),
            text: text.to_string(),
            bounds: (0, 0, 100, 100),
            clickable: true,
        }))
    }

    fn find_by_text_contains(&self, text: &str) -> Result<Option<Element>, CommandError> {
        Ok(Some(Element {
            id: "mock".to_string(),
            text: text.to_string(),
            bounds: (0, 0, 100, 100),
            clickable: true,
        }))
    }

    fn find_by_id(&self, id: &str) -> Result<Option<Element>, CommandError> {
        Ok(Some(Element {
            id: id.to_string(),
            text: String::new(),
            bounds: (0, 0, 100, 100),
            clickable: true,
        }))
    }

    fn find_by_class(&self, class: &str) -> Result<Option<Element>, CommandError> {
        Ok(Some(Element {
            id: "mock".to_string(),
            text: class.to_string(),
            bounds: (0, 0, 100, 100),
            clickable: true,
        }))
    }
}

/// UI 元素
#[derive(Debug, Clone)]
pub struct Element {
    pub id: String,
    pub text: String,
    pub bounds: (i32, i32, i32, i32), // x, y, width, height
    pub clickable: bool,
}

impl Element {
    pub fn center(&self) -> Point {
        Point {
            x: self.bounds.0 + self.bounds.2 / 2,
            y: self.bounds.1 + self.bounds.3 / 2,
        }
    }

    pub fn click(&self) -> Command {
        let center = self.center();
        Command::Click {
            x: center.x,
            y: center.y,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_element_finder() {
        let finder = ElementFinder::new();
        let result = finder.find(&Selector::Text("Login".to_string()));
        assert!(result.is_ok());
    }

    #[test]
    fn test_element_center() {
        let elem = Element {
            id: "test".to_string(),
            text: "Button".to_string(),
            bounds: (100, 200, 50, 30),
            clickable: true,
        };
        let center = elem.center();
        assert_eq!(center.x, 125);
        assert_eq!(center.y, 215);
    }
}
