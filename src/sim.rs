use crate::grid;
use crate::prelude::*;

pub struct World {
	w: f32,
	h: f32,
	t: f64,
	atoms: Vec<Atom>,
	f_buf: Vec<Vec2>,
}

impl World {
	pub fn new(w: f32, h: f32) -> World {
		World::with_capacity(w, h, 1024)
	}

	pub fn with_capacity(w: f32, h: f32, num_atoms: usize) -> World {
		World {
			t: 0.0,
			w: w,
			h: h,
			atoms: Vec::with_capacity(num_atoms),
			f_buf: Vec::with_capacity(num_atoms),
		}
	}

	pub fn w(&self) -> f32 {
		self.w
	}

	pub fn h(&self) -> f32 {
		self.h
	}

	pub fn push(&mut self, a: Atom) {
		self.atoms.push(a);
		self.f_buf.push(Vec2::zero());
	}

	pub fn atoms(&self) -> &Vec<Atom> {
		&self.atoms
	}

	pub fn update_steps(&mut self, steps: usize, dt: f32) {
		for _ in 0..steps {
			self.update(dt);
		}
	}

	fn update(&mut self, dt: f32) {
		//let f = f_brute(&self.atoms);

		let cell_size = 2.5;
		let f = grid::f_fast(&self.atoms, self.w, self.h, cell_size);

		self.update_pv(&f, dt);
		self.t += dt as f64;
		self.apply_box();
		self.apply_temp(dt);
	}

	/// Update the atom's position and velocity, given the force on each atom,
	/// using the Velocity Verlet integrator.
	fn update_pv(&mut self, f: &Vec<Vec2>, dt: f32) {
		for (i, a) in &mut self.atoms.iter_mut().enumerate() {
			a.v += f[i] * dt;
			a.p += a.v * dt;
		}
	}

	fn apply_box(&mut self) {
		let w = self.w;
		let h = self.h;

		for a in &mut self.atoms {
			if a.p[0] > w && a.v[0] > 0.0 {
				a.v[0] = -0.5 * a.v[0];
			}
			if a.p[0] < 0.0 && a.v[0] < 0.0 {
				a.v[0] = -0.5 * a.v[0];
			}
			if a.p[1] > h && a.v[1] > 0.0 {
				a.v[1] = -0.5 * a.v[1];
			}
			if a.p[1] < 0.0 && a.v[1] < 0.0 {
				a.v[1] = -0.5 * a.v[1];
			}
		}
	}

	fn apply_temp(&mut self, dt: f32) {
		for a in &mut self.atoms {
			a.v *= 1.0 - 0.002 * dt;
		}
	}
}

/// O(N^2) force calculation
fn f_brute(atoms: &Vec<Atom>) -> Vec<Vec2> {
	let mut acc = vec![Vec2::zero(); atoms.len()];
	for i in 0..atoms.len() {
		for j in (i + 1)..atoms.len() {
			let f = atoms[i].force(&atoms[j]);
			acc[i] += f;
			acc[j] -= f;
		}
	}
	acc
}
