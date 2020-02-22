pub use super::atom::*;
pub use super::vec2::*;

// linear interpolation
// 	x1 -> y1
// 	x2 -> y2
// 	x  -> y
pub fn linterp(x1: f32, y1: f32, x2: f32, y2: f32, x: f32) -> f32 {
	y1 + (y2 - y1) * (x - x1) / (x2 - x1)
}
