use crate::engine::{Point, Rect};

/// Calculate center point of a rectangle
pub fn rect_center(rect: &Rect) -> Point {
    Point {
        x: rect.x + (rect.width as i32) / 2,
        y: rect.y + (rect.height as i32) / 2,
    }
}

/// Check if a point is inside a rectangle
pub fn point_in_rect(point: &Point, rect: &Rect) -> bool {
    point.x >= rect.x
        && point.x <= rect.x + rect.width as i32
        && point.y >= rect.y
        && point.y <= rect.y + rect.height as i32
}

/// Calculate distance between two points
pub fn distance(p1: &Point, p2: &Point) -> f64 {
    let dx = (p2.x - p1.x) as f64;
    let dy = (p2.y - p1.y) as f64;
    (dx * dx + dy * dy).sqrt()
}

/// Check if two rectangles overlap
pub fn rects_overlap(r1: &Rect, r2: &Rect) -> bool {
    let r1_right = r1.x + r1.width as i32;
    let r1_bottom = r1.y + r1.height as i32;
    let r2_right = r2.x + r2.width as i32;
    let r2_bottom = r2.y + r2.height as i32;

    !(r1_right < r2.x || r2_right < r1.x || r1_bottom < r2.y || r2_bottom < r1.y)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_rect_center() {
        let rect = Rect {
            x: 0,
            y: 0,
            width: 100,
            height: 100,
        };
        let center = rect_center(&rect);
        assert_eq!(center.x, 50);
        assert_eq!(center.y, 50);
    }

    #[test]
    fn test_point_in_rect() {
        let rect = Rect {
            x: 0,
            y: 0,
            width: 100,
            height: 100,
        };
        assert!(point_in_rect(&Point { x: 50, y: 50 }, &rect));
        assert!(!point_in_rect(&Point { x: 150, y: 150 }, &rect));
    }
}
