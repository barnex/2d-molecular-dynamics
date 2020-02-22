extern crate rand;

use atoms::prelude::*;
use atoms::sim::World;
use std::time;

pub fn main() {
	// ---- set up world
	let width = 120.0;
	let height = 80.0;
	let mut w = World::new(width, height);

	// ---- initial state
	let mut n_atom = 0;
	let lattice = 1.00;
	for i in 0..32 {
		for j in 0..32 {
			n_atom += 1;
			let x = linterp(0.0, 0.0, 32.0, height * lattice, i as f32);
			let y = linterp(0.0, 0.0, 32.0, height * lattice, j as f32);
			w.push(Atom::new(x, y));
		}
	}

	let n_step = 1000;
	let dt = 0.002;
	let start = time::Instant::now();

	w.update_steps(n_step, dt);

	let wall = start.elapsed();

	println!(
		"{} ns/atom/step",
		((1e9 * wall.as_secs_f64()) / (n_atom * n_step) as f64) as i32,
	);
}
