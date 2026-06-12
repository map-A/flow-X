use crate::engine::{Command, Point};

/// 手势操作
pub struct Gesture;

impl Gesture {
    /// 捏合手势 (缩小)
    pub fn pinch_in(center: Point, distance: i32, duration_ms: u64) -> Vec<Command> {
        let half = distance / 2;
        vec![
            Command::Swipe {
                from: Point {
                    x: center.x - half,
                    y: center.y,
                },
                to: Point {
                    x: center.x - 10,
                    y: center.y,
                },
                duration_ms,
            },
            Command::Swipe {
                from: Point {
                    x: center.x + half,
                    y: center.y,
                },
                to: Point {
                    x: center.x + 10,
                    y: center.y,
                },
                duration_ms,
            },
        ]
    }

    /// 拉伸手势 (放大)
    pub fn pinch_out(center: Point, distance: i32, duration_ms: u64) -> Vec<Command> {
        let half = distance / 2;
        vec![
            Command::Swipe {
                from: Point {
                    x: center.x - 10,
                    y: center.y,
                },
                to: Point {
                    x: center.x - half,
                    y: center.y,
                },
                duration_ms,
            },
            Command::Swipe {
                from: Point {
                    x: center.x + 10,
                    y: center.y,
                },
                to: Point {
                    x: center.x + half,
                    y: center.y,
                },
                duration_ms,
            },
        ]
    }

    /// 旋转手势
    pub fn rotate(
        center: Point,
        angle_degrees: i32,
        radius: i32,
        duration_ms: u64,
    ) -> Vec<Command> {
        let angle_rad = (angle_degrees as f32).to_radians();
        let cos = angle_rad.cos();
        let sin = angle_rad.sin();

        let start_x = center.x + radius;
        let start_y = center.y;

        let end_x = center.x + (radius as f32 * cos) as i32;
        let end_y = center.y + (radius as f32 * sin) as i32;

        vec![Command::Swipe {
            from: Point {
                x: start_x,
                y: start_y,
            },
            to: Point { x: end_x, y: end_y },
            duration_ms,
        }]
    }

    /// 双击
    pub fn double_tap(x: i32, y: i32) -> Vec<Command> {
        vec![Command::Click { x, y }, Command::Click { x, y }]
    }

    /// 三指滑动
    pub fn three_finger_swipe(
        from: Point,
        to: Point,
        spacing: i32,
        duration_ms: u64,
    ) -> Vec<Command> {
        vec![
            Command::Swipe {
                from: Point {
                    x: from.x - spacing,
                    y: from.y,
                },
                to: Point {
                    x: to.x - spacing,
                    y: to.y,
                },
                duration_ms,
            },
            Command::Swipe {
                from: from.clone(),
                to: to.clone(),
                duration_ms,
            },
            Command::Swipe {
                from: Point {
                    x: from.x + spacing,
                    y: from.y,
                },
                to: Point {
                    x: to.x + spacing,
                    y: to.y,
                },
                duration_ms,
            },
        ]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_pinch_in() {
        let center = Point { x: 500, y: 500 };
        let cmds = Gesture::pinch_in(center, 200, 300);
        assert_eq!(cmds.len(), 2);
    }

    #[test]
    fn test_pinch_out() {
        let center = Point { x: 500, y: 500 };
        let cmds = Gesture::pinch_out(center, 200, 300);
        assert_eq!(cmds.len(), 2);
    }

    #[test]
    fn test_double_tap() {
        let cmds = Gesture::double_tap(100, 200);
        assert_eq!(cmds.len(), 2);
    }

    #[test]
    fn test_three_finger_swipe() {
        let from = Point { x: 500, y: 1000 };
        let to = Point { x: 500, y: 200 };
        let cmds = Gesture::three_finger_swipe(from, to, 100, 300);
        assert_eq!(cmds.len(), 3);
    }
}
