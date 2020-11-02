use delaunator::Point;

pub fn jitter(p: &Point, r: f64) -> Point {
    return Point {
        x: p.x + (p.x + p.y).sin() * r,
        y: p.y + (p.x - p.y).cos() * r,
    };
}
