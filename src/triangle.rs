use crate::fragment::Fragment;
use crate::vertex::Vertex;
use raylib::math::Vector3;

#[inline]
fn edge(ax: f32, ay: f32, bx: f32, by: f32, px: f32, py: f32) -> f32 {
    (px - ax) * (by - ay) - (py - ay) * (bx - ax)
}

pub fn triangle(v0: &Vertex, v1: &Vertex, v2: &Vertex) -> Vec<Fragment> {
    let mut frags = Vec::new();

    let p0 = v0.transformed_position;
    let p1 = v1.transformed_position;
    let p2 = v2.transformed_position;

    let area = edge(p0.x, p0.y, p1.x, p1.y, p2.x, p2.y);
    if area.abs() < 1e-6 || area > 0.0 {
        return frags;
    }
    let inv_area = 1.0 / area;

    let min_x = p0.x.min(p1.x).min(p2.x).floor() as i32;
    let max_x = p0.x.max(p1.x).max(p2.x).ceil() as i32;
    let min_y = p0.y.min(p1.y).min(p2.y).floor() as i32;
    let max_y = p0.y.max(p1.y).max(p2.y).ceil() as i32;

    for y in min_y..=max_y {
        for x in min_x..=max_x {
            let px = x as f32 + 0.5;
            let py = y as f32 + 0.5;

            let w0 = edge(p1.x, p1.y, p2.x, p2.y, px, py) * inv_area;
            let w1 = edge(p2.x, p2.y, p0.x, p0.y, px, py) * inv_area;
            let w2 = 1.0 - w0 - w1;

            if w0 >= 0.0 && w1 >= 0.0 && w2 >= 0.0 {
    let depth = p0.z * w0 + p1.z * w1 + p2.z * w2;
    let color =
        v0.color * w0 +
        v1.color * w1 +
        v2.color * w2;
    frags.push(Fragment::new(px, py, color, depth));
}
        }
    }

    frags
}
