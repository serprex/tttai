use std::iter::{IntoIterator, Iterator};

pub trait Player {
	fn mv(&mut self, Game) -> u8;
	fn feedback(&mut self, i32) -> () {}
}

#[derive(Clone, Copy, Hash, PartialEq, Eq)]
pub enum Spot {
	X, O, A
}

#[derive(Clone, Copy, Hash, PartialEq, Eq)]
pub enum GameResult {
	X, O, OX
}

#[derive(Clone, Copy, Hash, PartialEq, Eq)]
pub struct Game(u32);

pub struct GameIter(u32);

impl IntoIterator for Game {
	type Item = Spot;
	type IntoIter = GameIter;
	fn into_iter(self) -> Self::IntoIter {
		GameIter(self.0|0x40000)
	}
}

impl Iterator for GameIter {
	type Item = Spot;
	fn next(&mut self) -> Option<Self::Item> {
		if self.0 == 1 { None }
		else {
			let result = match self.0 & 3 {
				0 => Spot::X,
				3 => Spot::O,
				_ => Spot::A,
			};
			self.0 >>= 2;
			Some(result)
		}
	}
}

impl Game {
	pub fn new() -> Self {
		Game(0x2AAAA)
	}

	pub fn set(&mut self, idx: u8, val: Spot) -> () {
		self.0 &= !(3 << idx*2);
		self.0 |= match val {
			Spot::X => return,
			Spot::O => 3,
			Spot::A => 2,
		} << idx*2
	}

	pub fn get(&self, idx: u8) -> Spot {
		match (self.0 >> idx*2) & 3 {
			0 => Spot::X,
			3 => Spot::O,
			_ => Spot::A,
		}
	}

	pub fn prgame(self) -> () {
		let mut out = String::new();
		for (i, spot) in self.into_iter().enumerate() {
			out.push(match spot {
				Spot::O => 'O',
				Spot::X => 'X',
				Spot::A => '-',
			});
			if i%3 == 2 { out.push('\n') }
		}
		print!("{}", out)
	}

	pub fn flip_board(&mut self) -> () {
		self.0 = !self.0 & 0x3ffff
	}

	pub fn x_wins(&self) -> GameResult {
		if (self.0 & 0x3f) == 0 ||
			(self.0 & 0xfc0) == 0 ||
			(self.0 & 0x3f00) == 0 ||
			(self.0 & 0x30c3) == 0 ||
			(self.0 & 0x3c0c) == 0 ||
			(self.0 & 0x30c30) == 0 ||
			(self.0 & 0x30303) == 0 ||
			(self.0 & 0x3330) == 0 { GameResult::X }
		else if self.into_iter().any(|x| x == Spot::A) { GameResult::O }
		else { GameResult::OX }
	}
}

pub fn play<P1, P2>(p1: &mut P1, p2: &mut P2, mut player: bool, prwin: bool) -> GameResult
	where P1: Player, P2: Player {
	let mut game = Game::new();
	loop {
		let mv = if player { p2.mv(game) } else { p1.mv(game) };
		if mv > 8 || game.get(mv) != Spot::A {
			if prwin {
				println!("Illegal move forfeit");
				game.prgame();
			}
			return if player { GameResult::X } else { GameResult::O }
		}
		game.set(mv, Spot::X);
		let winner = game.x_wins();
		if winner != GameResult::O {
			if prwin {
				game.prgame();
			}
			return if winner == GameResult::X {
				if player { GameResult::O } else { GameResult::X }
			} else { winner }
		}
		game.flip_board();
		player ^= true
	}
}
