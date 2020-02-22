use crate::prelude::*;
/*
pub struct World {
	pub atoms: Vec<Atom>,
	pub t: f64,
	grid: Grid,
}

impl World {
	pub fn new(cells_x: usize, cells_y: usize, cell_size: f64) -> World {
		World {
			atoms: vec![],
			t: 0.0,
			grid: Grid::new(cells_x, cells_y, cell_size),
		}
	}

	pub fn push(&mut self, a: Atom) {
		let mut a = a;
		a.p[0] = clamp(a.p[0], 0.0, self.w());
		a.p[1] = clamp(a.p[1], 0.0, self.h());
		self.atoms.push(a)
	}

	pub fn update_steps(&mut self, steps: usize, dt: f32) {
		for _ in 0..steps {
			self.update(dt);
		}
	}

	pub fn update(&mut self, dt: f32) {
		self.apply_box();
		self.update_a();

		// velocity Verlet integration
		for a in &mut self.atoms {
			a.v += a.a * dt;
			a.p += a.v * dt;
		}

		self.t += dt as f64;
	}

	fn update_a(&mut self) {
		// TODO: Grid should do this
		for a in &mut self.atoms {
			a.a = Vec2::zero();
		}

		self.grid.update_a1(&self.atoms);
		self.grid.update_a2(&mut self.atoms);
	}

	pub fn w(&self) -> f32 {
		self.grid.w()
	}

	pub fn h(&self) -> f32 {
		self.grid.h()
	}

	fn apply_box(&mut self) {
		let w = self.w();
		let h = self.h();

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
		//for a in &mut self.atoms {
		//	a.v *= 1.0 - 0.001 * dt;
		//}
	}
}

struct Grid {
	cells_x: i32,
	cells_y: i32,
	cell_size: f32,
	cells: Vec<Cell>,
}

impl Grid {
	fn new(cells_x: usize, cells_y: usize, cell_size: f64) -> Grid {
		let num_cells = cells_x * cells_y;
		let mut cells = Vec::with_capacity(num_cells);
		let cap = 16;
		for _ in 0..num_cells {
			cells.push(Cell::with_capacity(cap));
		}
		Grid {
			cells_x: cells_x as i32,
			cells_y: cells_y as i32,
			cell_size: cell_size as f32,
			cells: cells,
		}
	}

	fn update_a1(&mut self, atoms: &Vec<Atom>) {
		// reset
		for c in &mut self.cells {
			c.clear();
		}

		// copy atoms to cells
		for a in atoms {
			let (x, y) = (a.p[0], a.p[1]);
			let mut i = (x / self.cell_size) as i32;
			let mut j = (y / self.cell_size) as i32;

			if i < 0 {
				i = 0;
			}
			if j < 0 {
				j = 0;
			}
			if i >= self.cells_x {
				i = self.cells_x - 1;
			}
			if j >= self.cells_y {
				j = self.cells_y - 1;
			}

			let cell_index = j * self.cells_x + i;
			if cell_index < 0 || cell_index >= self.cells.len() as i32 {
				panic!("pos: {}, (i,j): ({},{}), index: {}", a.p, i, j, cell_index);
			}

			self.cells[cell_index as usize].push(*a);
		}

		// update forces
		for iy in 0..self.cells_y {
			for ix in 0..self.cells_x {
				self.update_a_cell(ix, iy);
			}
		}
	}

	fn update_a2(&self, atoms: &mut Vec<Atom>) {
		// copy atoms back
		let mut i = 0;
		for c in &self.cells {
			for a in c {
				atoms[i] = *a;
				i += 1;
			}
		}
	}

	fn update_a_cell(&mut self, my_x: i32, my_y: i32) {
		let my_index = self.cell_index(my_x, my_y);
		self.update_a_self(my_index);

		for delta_y in -1..2 {
			let their_y = my_y + delta_y;
			if their_y < 0 || their_y >= self.cells_y {
				continue;
			}
			for delta_x in -1..2 {
				let their_x = my_x + delta_x;
				if their_x < 0 || their_x >= self.cells_x {
					continue;
				}
				let their_index = self.cell_index(their_x, their_y);
				if their_index <= my_index {
					continue;
				}
				self.update_a_neigh(my_index, their_index);
			}
		}
	}

	fn update_a_self(&mut self, cell_index: usize) {
		let atoms = &mut self.cells[cell_index];
		for i in 0..atoms.len() {
			for j in (i + 1)..atoms.len() {
				let f = atoms[i].force(&atoms[j]);
				atoms[i].a += f;
				atoms[j].a -= f;
			}
		}
	}

	fn update_a_neigh(&mut self, my_index: usize, their_index: usize) {
		for mine in &mut self.cells[my_index] {
			for theirs in &mut self.cells[their_index] {
				let f = mine.force(theirs);
				mine.a += f;
				theirs.a -= f;
			}
		}
	}

	fn cell_of(&self, p: Vec2) -> usize {
		let (x, y) = (p[0], p[1]);
		let (w, h) = (self.w(), self.h());
		let i = ((x + w / 2.0) / self.cell_size) as i32;
		let j = ((y + h / 2.0) / self.cell_size) as i32;
		self.cell_index(i, j)
	}

	fn cell_index(&self, ix: i32, iy: i32) -> usize {
		debug_assert!(ix >= 0);
		debug_assert!(ix < self.cells_x);
		debug_assert!(iy >= 0);
		debug_assert!(iy < self.cells_y);
		(iy * self.cells_x + ix) as usize
	}

	pub fn w(&self) -> f32 {
		self.cells_x as f32 * self.cell_size
	}

	pub fn h(&self) -> f32 {
		self.cells_y as f32 * self.cell_size
	}
}


fn clamp<T: PartialOrd>(x: T, min: T, max: T) -> T {
	if x < min {
		return min;
	}
	if x > max {
		return max;
	}
	x
}
*/
