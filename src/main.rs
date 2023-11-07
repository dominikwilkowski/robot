use std::println;

enum Dir {
	N,
	W,
	E,
	S,
}

#[derive(Debug)]
struct Wearhouse {
	position: (u8, u8),
	size: (u8, u8),
}

impl Wearhouse {
	fn new(position: (u8, u8), size: (u8, u8)) -> Self {
		Self { position, size }
	}

	fn draw(&self) {
		println!("┌{:─^width$}┐", "", width = &self.size.0.into());
		for row in 0..self.size.1.into() {
			print!("│");
			for col in 0..self.size.0.into() {
				if self.position.0 == row && self.position.1 == col {
					print!("X");
				} else {
					print!(" ");
				}
			}
			println!("│");
		}
		println!("└{:─^width$}┘", "─", width = &self.size.0.into());
		println!("{:?}", &self.position);
	}

	fn moving(&mut self, dir: Dir) {
		match dir {
			Dir::N => {
				if self.position.0 != 0 {
					self.position.0 -= 1;
				}
			}
			Dir::W => {
				if self.position.1 != 0 {
					self.position.1 -= 1;
				}
			}
			Dir::E => {
				if self.position.1 != self.size.1 {
					self.position.1 += 1;
				}
			}
			Dir::S => {
				if self.position.0 != self.size.0 {
					self.position.0 += 1;
				}
			}
		}
	}
}

fn main() {
	let mut my_wearhouse = Wearhouse::new((2, 2), (10, 10));
	my_wearhouse.draw();
	my_wearhouse.moving(Dir::N);
	my_wearhouse.draw();
	my_wearhouse.moving(Dir::S);
	my_wearhouse.draw();
	my_wearhouse.moving(Dir::W);
	my_wearhouse.draw();
	my_wearhouse.moving(Dir::W);
	my_wearhouse.draw();
	my_wearhouse.moving(Dir::E);
	my_wearhouse.draw();
}

#[cfg(test)]
mod testing {
	use super::*;

	#[test]
	fn setting_size() {
		let my_wearhouse = Wearhouse::new((0, 0), (10, 10));
		assert_eq!(my_wearhouse.size, (10, 10));

		let my_wearhouse = Wearhouse::new((0, 0), (12, 12));
		assert_eq!(my_wearhouse.size, (12, 12));

		let my_wearhouse = Wearhouse::new((0, 0), (7, 14));
		assert_eq!(my_wearhouse.size, (7, 14));
	}

	#[test]
	fn moving_north() {
		let mut my_wearhouse = Wearhouse::new((5, 5), (10, 10));
		my_wearhouse.moving(Dir::N);
		assert_eq!(my_wearhouse.position, (4, 5));
		my_wearhouse.moving(Dir::N);
		assert_eq!(my_wearhouse.position, (3, 5));
		my_wearhouse.moving(Dir::N);
		assert_eq!(my_wearhouse.position, (2, 5));
		my_wearhouse.moving(Dir::N);
		assert_eq!(my_wearhouse.position, (1, 5));
		my_wearhouse.moving(Dir::N);
		assert_eq!(my_wearhouse.position, (0, 5));
		my_wearhouse.moving(Dir::N);
		assert_eq!(my_wearhouse.position, (0, 5));
		my_wearhouse.moving(Dir::N);
		assert_eq!(my_wearhouse.position, (0, 5));
	}

	#[test]
	fn moving_south() {
		let mut my_wearhouse = Wearhouse::new((5, 5), (10, 10));
		my_wearhouse.moving(Dir::S);
		assert_eq!(my_wearhouse.position, (6, 5));
		my_wearhouse.moving(Dir::S);
		assert_eq!(my_wearhouse.position, (7, 5));
		my_wearhouse.moving(Dir::S);
		assert_eq!(my_wearhouse.position, (8, 5));
		my_wearhouse.moving(Dir::S);
		assert_eq!(my_wearhouse.position, (9, 5));
		my_wearhouse.moving(Dir::S);
		assert_eq!(my_wearhouse.position, (10, 5));
		my_wearhouse.moving(Dir::S);
		assert_eq!(my_wearhouse.position, (10, 5));
		my_wearhouse.moving(Dir::S);
		assert_eq!(my_wearhouse.position, (10, 5));
	}

	#[test]
	fn moving_west() {
		let mut my_wearhouse = Wearhouse::new((5, 5), (10, 10));
		my_wearhouse.moving(Dir::W);
		assert_eq!(my_wearhouse.position, (5, 4));
		my_wearhouse.moving(Dir::W);
		assert_eq!(my_wearhouse.position, (5, 3));
		my_wearhouse.moving(Dir::W);
		assert_eq!(my_wearhouse.position, (5, 2));
		my_wearhouse.moving(Dir::W);
		assert_eq!(my_wearhouse.position, (5, 1));
		my_wearhouse.moving(Dir::W);
		assert_eq!(my_wearhouse.position, (5, 0));
		my_wearhouse.moving(Dir::W);
		assert_eq!(my_wearhouse.position, (5, 0));
		my_wearhouse.moving(Dir::W);
		assert_eq!(my_wearhouse.position, (5, 0));
	}

	#[test]
	fn moving_east() {
		let mut my_wearhouse = Wearhouse::new((5, 5), (10, 10));
		my_wearhouse.moving(Dir::E);
		assert_eq!(my_wearhouse.position, (5, 6));
		my_wearhouse.moving(Dir::E);
		assert_eq!(my_wearhouse.position, (5, 7));
		my_wearhouse.moving(Dir::E);
		assert_eq!(my_wearhouse.position, (5, 8));
		my_wearhouse.moving(Dir::E);
		assert_eq!(my_wearhouse.position, (5, 9));
		my_wearhouse.moving(Dir::E);
		assert_eq!(my_wearhouse.position, (5, 10));
		my_wearhouse.moving(Dir::E);
		assert_eq!(my_wearhouse.position, (5, 10));
		my_wearhouse.moving(Dir::E);
		assert_eq!(my_wearhouse.position, (5, 10));
	}
}
