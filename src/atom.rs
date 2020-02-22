use crate::prelude::*;

pub struct Atom {
	pub p: Vec2, // position
	pub v: Vec2, // speed
	pub m: f32,  // mass
}

impl Atom {
	#[inline]
	pub fn new(x: f32, y: f32) -> Atom {
		Atom::with_v(x, y, 0.0, 0.0)
	}

	pub fn with_v(x: f32, y: f32, vx: f32, vy: f32) -> Atom {
		Atom {
			p: vec2(x, y),
			v: vec2(vx, vy),
			m: 1.0,
		}
	}

	#[inline]
	pub fn force(&self, b: &Atom) -> Vec2 {
		force(self.p, b.p)
	}
}

//#[inline]
//pub fn force(p1: Vec2, p2: Vec2) -> Vec2 {
//	let delta = p1 - p2;
//	let r = delta.len();
//	let f = f_lj(r);
//	delta.normalized() * f
//}

#[inline]
pub fn force(p1: Vec2, p2: Vec2) -> Vec2 {
	let delta = p1 - p2;
	let r2 = 1.0 / delta.len2();
	let r4 = r2 * r2;
	let r8 = r4 * r4;
	let r14 = r8 * r4 * r2;
	delta * (48.0 * r14 - 24.0 * r8)
}

fn f_lj(r: f32) -> f32 {
	let r = 1.0 / r;
	let r2 = r * r;
	let r3 = r2 * r;
	let r6 = r3 * r3;
	let r7 = r6 * r;
	let r13 = r6 * r7;

	48.0 * r13 - 24.0 * r7
}

/// https://en.wikipedia.org/wiki/Lennard-Jones_potential
fn e_lj(r: f32) -> f32 {
	e_lj2(r * r)
}

fn e_lj2(r2: f32) -> f32 {
	let s2 = 1.0 / r2;
	let s6 = s2 * s2 * s2;
	let s12 = s6 * s6;

	4.0 * (s12 - s6)
}
