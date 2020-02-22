use std::cmp::PartialEq;
use std::fmt;
use std::ops::*;

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Vec2 {
	el: [f32; 2],
}

#[inline]
pub fn vec2(x: f32, y: f32) -> Vec2 {
	Vec2::new(x, y)
}

impl Vec2 {
	#[inline]
	pub fn new(x: f32, y: f32) -> Self {
		Self { el: [x, y] }
	}

	#[inline]
	pub fn fmadd(self, t: f32, b: Self) -> Self {
		self + (b * t)
	}

	#[inline]
	pub fn dot(self, rhs: Self) -> f32 {
		self[0] * rhs[0] + self[1] * rhs[1]
	}

	#[inline]
	/// Len returns the length (norm).
	pub fn len(self) -> f32 {
		self.dot(self).sqrt()
	}

	#[inline]
	/// Len2 returns the length squared.
	pub fn len2(self) -> f32 {
		self.dot(self)
	}

	/// Normalized returns a vector with the same direction
	/// but unit length.
	#[inline]
	pub fn normalized(self) -> Self {
		self * (1. / self.len())
	}

	#[inline]
	pub fn normalize(&mut self) {
		*self = self.normalized()
	}

	pub fn is_normalized(&self) -> bool {
		(self.len() - 1.0).abs() < 1e-6
	}

	/// Shorthand for new(0.0, 0.0, 0.0).
	#[inline]
	pub fn zero() -> Vec2 {
		Vec2 { el: [0.0, 0.0] }
	}
}

impl Add<Vec2> for Vec2 {
	type Output = Self;

	#[inline]
	fn add(self, rhs: Vec2) -> Self::Output {
		Self {
			el: [self[0] + rhs[0], self[1] + rhs[1]],
		}
	}
}

impl AddAssign<Vec2> for Vec2 {
	#[inline]
	fn add_assign(&mut self, rhs: Self) {
		self[0] += rhs[0];
		self[1] += rhs[1];
	}
}

impl fmt::Display for Vec2 {
	fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
		write!(f, "[{}, {}]", self[0], self[1])
	}
}

impl Index<usize> for Vec2 {
	type Output = f32;
	#[inline]
	fn index(&self, idx: usize) -> &Self::Output {
		&self.el[idx]
	}
}

impl IndexMut<usize> for Vec2 {
	#[inline]
	fn index_mut(&mut self, idx: usize) -> &mut Self::Output {
		&mut self.el[idx]
	}
}

impl Mul<f32> for Vec2 {
	type Output = Self;

	#[inline]
	fn mul(self, rhs: f32) -> Self::Output {
		Self {
			el: [self[0] * rhs, self[1] * rhs],
		}
	}
}

impl MulAssign<f32> for Vec2 {
	#[inline]
	fn mul_assign(&mut self, rhs: f32) {
		self[0] *= rhs;
		self[1] *= rhs;
	}
}

impl Mul<Vec2> for f32 {
	type Output = Vec2;
	#[inline]
	fn mul(self, rhs: Vec2) -> Self::Output {
		rhs.mul(self)
	}
}

impl Neg for Vec2 {
	type Output = Self;
	#[inline]
	fn neg(self) -> Self::Output {
		Self {
			el: [-self.el[0], -self.el[1]],
		}
	}
}

impl Sub for Vec2 {
	type Output = Self;

	#[inline]
	fn sub(self, rhs: Vec2) -> Self::Output {
		Self {
			el: [self[0] - rhs[0], self[1] - rhs[1]],
		}
	}
}

impl SubAssign for Vec2 {
	#[inline]
	fn sub_assign(&mut self, rhs: Self) {
		self[0] -= rhs[0];
		self[1] -= rhs[1];
	}
}

#[test]
fn test_add() {
	let a = vec2(1.0, 2.0);
	let b = vec2(2.0, 3.0);
	assert_eq!(a + b, vec2(3.0, 5.0));
	// check that unsafe code did not accidentaly overwrite
	assert_eq!(a, vec2(1.0, 2.0));
	assert_eq!(b, vec2(2.0, 3.0));
}

#[test]
fn test_add_assign() {
	let mut a = vec2(1.0, 2.0);
	let b = vec2(2.0, 3.0);
	a += b;
	assert_eq!(a, vec2(3.0, 5.0));
	// check that unsafe code did not accidentaly overwrite
	assert_eq!(b, vec2(2.0, 3.0));
}

#[test]
fn test_dot() {
	let a = vec2(1.0, 5.0);
	let b = vec2(-2.0, 1.0);
	assert_eq!(a.dot(b), 3.0)
}

#[test]
fn test_eq() {
	let a = vec2(1.0, 2.0);
	let b = vec2(2.0, 3.0);
	assert_eq!(a, a);
	assert!(a != b);
}

#[test]
fn test_fmadd() {
	let a = vec2(1., 2.);
	let b = vec2(4., 5.);
	assert_eq!(a.fmadd(2., b), vec2(9., 12.));
	// check that unsafe code did not accidentaly overwrite
	assert_eq!(a, vec2(1., 2.,));
	assert_eq!(b, vec2(4., 5.,));
}

#[test]
fn test_index() {
	let a = vec2(1.0, 2.0);
	assert_eq!(a[0], 1.0);
	assert_eq!(a[1], 2.0);
}

#[test]
fn test_index_mut() {
	let mut a = vec2(0.0, 0.0);
	a[0] = 1.0;
	a[1] = 2.0;
	assert_eq!(a[0], 1.0);
	assert_eq!(a[1], 2.0);
}

#[test]
fn test_len() {
	let a = vec2(3.0, 4.0);
	assert_eq!(a.len(), 5.0);
}

#[test]
fn test_mul() {
	{
		let a = vec2(1.0, 2.0);
		assert_eq!(a * 2.0, vec2(2.0, 4.0));
		assert_eq!(2.0 * a, vec2(2.0, 4.0));
		// check that unsafe code did not accidentaly overwrite
		assert_eq!(a, vec2(1.0, 2.0));
	}
	{
		let b = vec2(1.0, 2.0);
		assert_eq!(b * 2.0, vec2(2.0, 4.0));
		assert_eq!(2.0 * b, vec2(2.0, 4.0));
		assert_eq!(b, vec2(1.0, 2.0));
	}
}

#[test]
fn test_mul_assign() {
	let mut a = vec2(1.0, 2.0);
	a *= 2.0;
	assert_eq!(a, vec2(2.0, 4.0));
}

#[test]
fn test_neg() {
	let a = vec2(1.0, 2.0);
	assert_eq!(-a, vec2(-1.0, -2.0));
	// check that unsafe code did not accidentaly overwrite
	assert_eq!(a, vec2(1.0, 2.0));
}

#[test]
fn test_sub() {
	let a = vec2(1.0, 2.0);
	let b = vec2(2.0, 4.0);
	assert_eq!(a - b, vec2(-1.0, -2.0));
	// check that unsafe code did not accidentaly overwrite
	assert_eq!(a, vec2(1.0, 2.0,));
	assert_eq!(b, vec2(2.0, 4.0,));
}

#[test]
fn test_sub_assign() {
	let mut a = vec2(1.0, 2.0);
	let b = vec2(2.0, 4.0);
	a -= b;
	assert_eq!(a, vec2(-1.0, -2.0));
	// check that unsafe code did not accidentaly overwrite
	assert_eq!(b, vec2(2.0, 4.0,));
}
