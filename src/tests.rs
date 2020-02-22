use crate::prelude::*;

#[test]
fn test_force() {
	let a = Atom::new(-1.0, 0.0);
	let b = Atom::new(1.0, 0.0);
	assert_eq!(a.force(&b), Vec2::new(0.18164063, 0.0));
}
