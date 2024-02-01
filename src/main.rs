use std::collections::HashMap;

struct Board {
	internal: HashMap<usize, HashMap<usize, Cell>>,
	future: HashMap<usize, HashMap<usize, Cell>>,
}

impl Board {
	pub fn new() -> Self {
		Self {
			internal: HashMap::new(),
			future: HashMap::new(),
		}
	}
}

struct Cell {
	life_force: u8,
}

impl Cell {
	// least abstract
	pub fn life_force_getter(&self) -> u8 {
		self.life_force
	}
	pub fn life_force_setter(&mut self, n: u8) {
		self.life_force = n;
	}
}

fn main() {
	let mut board = Board::new();
	
}