use crate::atom;
use crate::prelude::*;

pub fn f_fast(atoms: &Vec<Atom>, w: f32, h: f32, s: f32) -> Vec<Vec2> {
	let n_x = (w / s) as usize + 1; // TODO: inaccurate rounding
	let n_y = (h / s) as usize + 1;
	let mut grid = Grid::new(n_x, n_y, s);
	grid.copy(atoms);
	grid.force(atoms.len())
}

struct Grid {
	n_x: i32,
	n_y: i32,
	cell_size: f32,
	cells: Vec<Cell>,
}

type Cell = Vec<(u32, Vec2)>;

impl Grid {
	fn new(n_x: usize, n_y: usize, cell_size: f32) -> Grid {
		let n_cell = n_x * n_y;
		let mut cells = Vec::with_capacity(n_cell);
		for _ in 0..n_cell {
			cells.push(Cell::with_capacity(16));
		}
		Grid {
			n_x: n_x as i32,
			n_y: n_y as i32,
			cell_size: cell_size as f32,
			cells: cells,
		}
	}

	/// copy atoms into cells
	fn copy(&mut self, atoms: &Vec<Atom>) {
		for (atom_i, a) in atoms.iter().enumerate() {
			let (x, y) = (a.p[0], a.p[1]);

			let i = clamp((x / self.cell_size) as i32, 0, self.n_x);
			let j = clamp((y / self.cell_size) as i32, 0, self.n_y);

			let cell_index = j * self.n_x + i;
			if cell_index < 0 || cell_index >= self.cells.len() as i32 {
				panic!("pos: {}, (i,j): ({},{}), index: {}", a.p, i, j, cell_index);
			}
			self.cells[cell_index as usize].push((atom_i as u32, a.p));
		}
	}

	pub fn force(&self, n_atoms: usize) -> Vec<Vec2> {
		let mut f = vec![Vec2::zero(); n_atoms];
		for iy in 0..self.n_y {
			for ix in 0..self.n_x {
				self.add_f_cell(&mut f, ix, iy);
			}
		}
		f
	}

	fn add_f_cell(&self, f: &mut Vec<Vec2>, my_x: i32, my_y: i32) {
		let my_index = self.cell_index(my_x, my_y);
		self.add_f_self(f, my_index);

		for delta_y in -1..2 {
			let their_y = my_y + delta_y;
			if their_y < 0 || their_y >= self.n_y {
				continue;
			}
			for delta_x in -1..2 {
				let their_x = my_x + delta_x;
				if their_x < 0 || their_x >= self.n_x {
					continue;
				}
				let their_index = self.cell_index(their_x, their_y);
				if their_index <= my_index {
					continue;
				}
				self.add_f_neigh(f, my_index, their_index);
			}
		}
	}

	fn add_f_self(&self, f: &mut Vec<Vec2>, cell_index: usize) {
		let atoms = &self.cells[cell_index];
		for i in 0..atoms.len() {
			for j in (i + 1)..atoms.len() {
				let (i1, p1) = atoms[i];
				let (i2, p2) = atoms[j];
				let F = atom::force(p1, p2);
				f[i1 as usize] += F;
				f[i2 as usize] -= F;
			}
		}
	}

	fn add_f_neigh(&self, f: &mut Vec<Vec2>, my_index: usize, their_index: usize) {
		for (i1, p1) in &self.cells[my_index] {
			for (i2, p2) in &self.cells[their_index] {
				let F = atom::force(*p1, *p2);
				f[*i1 as usize] += F;
				f[*i2 as usize] -= F;
			}
		}
	}

	fn cell_index(&self, ix: i32, iy: i32) -> usize {
		debug_assert!(ix >= 0);
		debug_assert!(ix < self.n_x);
		debug_assert!(iy >= 0);
		debug_assert!(iy < self.n_y);
		(iy * self.n_x + ix) as usize
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
