extern crate rand;

use std::collections::HashMap;
use std::collections::hash_map::Entry;
use std::io;
use rand::{thread_rng, Rng, Rand};

#[derive(Clone, Copy, Hash, PartialEq, Eq)]
enum Spot {
	O, X, A
}

#[derive(Clone, Copy, Hash, PartialEq, Eq)]
enum GameResult {
	O, X, A, OX
}

trait Player {
	fn mv(&mut self, [Spot; 9]) -> u8;
	fn feedback(&mut self, i32) -> () {}
}

struct Ai<R: Rng> {
	rng: R,
	lut: HashMap<[Spot; 9], [[u32; 2]; 9]>,
	choices: Vec<([Spot; 9], u8)>,
}

impl<R: Rng> Ai<R> {
	pub fn new(rng: R) -> Self {
		Ai {
			rng: rng,
			lut: Default::default(),
			choices: Default::default(),
		}
	}
}

struct RngAi<R: Rng> {
	rng: R,
}

impl<R: Rng> RngAi<R> {
	pub fn new(rng: R) -> Self {
		RngAi {
			rng: rng,
		}
	}
}

struct Human;
impl Player for Human {
	fn mv(&mut self, b: [Spot; 9]) -> u8 {
		prgame(b);
		let stdin = io::stdin();
		let mut line = String::new();
		stdin.read_line(&mut line);
		return match line.trim().parse::<u8>() {
			Ok(i) if i<9 => i,
			_ => self.mv(b),
		}
	}
}

fn prgame(b: [Spot; 9]) -> () {
	let mut out = String::new();
	for (i, &spot) in b.iter().enumerate() {
		out.push(match spot {
			Spot::O => 'O',
			Spot::X => 'X',
			Spot::A => '-',
		});
		if i%3 == 2 { out.push('\n') }
	}
	print!("{}", out)
}

impl<R: Rng> Player for Ai<R> {
	fn mv(&mut self, b: [Spot; 9]) -> u8 {
		let ent = self.lut.entry(b);
		let choice = match ent {
			Entry::Occupied(val) => {
				let mut max = isize::min_value();
				let mut maxi: [u8; 9] = Default::default();
				let mut mxidx = 0;
				for (i, wl) in val.get().iter().enumerate() {
					if b[i] != Spot::A { continue }
					let w = (wl[0] as isize) - (wl[1] as isize);
					if w > max {
						max = w;
						maxi[0] = i as u8;
						mxidx = 1;
					} else if w+3 > max {
						maxi[mxidx] = i as u8;
						mxidx += 1;
					}
				}
				*self.rng.choose(&maxi[..mxidx]).unwrap_or(&0)
			},
			Entry::Vacant(val) => {
				val.insert(Default::default());
				for (i, &spot) in b.iter().enumerate() {
					if spot == Spot::A { return i as u8 }
				}
				0
			}
		};
		self.choices.push((b, choice as u8));
		choice
	}
	fn feedback(&mut self, scale: i32) {
		if scale == 0 {
			return self.choices.clear()
		}
		let cidx = if scale > 0 { 0 } else { 1 };
		let scale = scale.abs() as u32;
		for (key, choice) in self.choices.drain(0..) {
			let ent = self.lut.entry(key);
			match ent {
				Entry::Occupied(mut val) => {
					let mut newval = val.get_mut();
					newval[choice as usize][cidx] += scale;
				},
				Entry::Vacant(val) => {
					let mut newval: [[u32; 2]; 9] = Default::default();
					newval[choice as usize][cidx] = scale;
					val.insert(newval);
				},
			}
		}
	}
}

impl<R: Rng> Player for RngAi<R> {
	fn mv(&mut self, b: [Spot; 9]) -> u8 {
		let mut icand: [u8; 9] = Default::default();
		let mut icanlen: usize = 0;
		for (i, &spot) in b.iter().enumerate() {
			if spot == Spot::A {
				icand[icanlen] = i as u8;
				icanlen += 1;
			}
		}
		*self.rng.choose(&icand[..icanlen]).unwrap_or(&0)
	}
}

fn flip_board(b: &mut [Spot; 9]) -> () {
	for a in b.iter_mut() {
		*a = match *a {
			Spot::O => Spot::X,
			Spot::X => Spot::O,
			Spot::A => Spot::A,
		}
	}
}

fn x_wins(b: &[Spot; 9]) -> GameResult {
	match (b[0], b[1], b[2], b[3], b[4], b[5], b[6], b[7], b[8]) {
		(Spot::X, Spot::X, Spot::X, _, _, _, _, _, _) |
		(_, _, _, Spot::X, Spot::X, Spot::X, _, _, _) |
		(_, _, _, _, _, _, Spot::X, Spot::X, Spot::X) |
		(Spot::X, _, _, Spot::X, _, _, Spot::X, _, _) |
		(_, Spot::X, _, _, Spot::X, _, _, Spot::X, _) |
		(_, _, Spot::X, _, _, Spot::X, _, _, Spot::X) |
		(Spot::X, _, _, _, Spot::X, _, _, _, Spot::X) |
		(_, _, Spot::X, _, Spot::X, _, Spot::X, _, _) => GameResult::X,
		_ => if b.contains(&Spot::A) { GameResult::A } else { GameResult::OX },
	}
}

fn play<P1, P2>(p1: &mut P1, p2: &mut P2, mut player: bool, prwin: bool) -> GameResult
	where P1: Player, P2: Player {
	let mut game = [Spot::A; 9];
	loop {
		let mv = if player { p2.mv(game) } else { p1.mv(game) } as usize;
		if mv > 8 || game[mv] != Spot::A {
			if prwin {
				println!("Illegal move forfeit");
				prgame(game);
			}
			return if player { GameResult::X } else { GameResult::O }
		}
		game[mv] = Spot::X;
		let winner = x_wins(&game);
		if winner != GameResult::A {
			if prwin {
				prgame(game);
			}
			return if winner == GameResult::X {
				if player { GameResult::O } else { GameResult::X }
			} else { winner }
		}
		flip_board(&mut game);
		player ^= true
	}
}

fn main() {	
	let mut trng = thread_rng();
	let mut rng = rand::XorShiftRng::rand(&mut trng);
	let mut ai1 = Ai::new(rand::XorShiftRng::rand(&mut trng));
	let mut ai2 = Ai::new(rand::XorShiftRng::rand(&mut trng));
	let mut ai3 = RngAi::new(rand::XorShiftRng::rand(&mut trng));
	let mut games: usize = 0;
	let mut totgames: usize = 0;
	loop {
		let first = rng.gen::<bool>();
		if games == 0 {
			println!("{} begins", if first { 'O' } else { 'X' });
		}
		let winner = if totgames%400000 == 0 {
			play(&mut ai1, &mut Human, first, false)
		} else if games&4 == 1 {
			play(&mut ai1, &mut ai3, first, games == 0)
		} else {
			play(&mut ai1, &mut ai2, first, games == 0)
		};
		match winner {
			GameResult::X => {
				ai1.feedback(2);
				ai2.feedback(-2);
				if games == 0 { println!("X wins") }
			},
			GameResult::O => {
				ai2.feedback(-2);
				ai1.feedback(2);
				if games == 0 { println!("O wins") }
			},
			GameResult::OX => {
				ai1.feedback(1);
				ai2.feedback(1);
				if games == 0 { println!("Draw") }
			},
			GameResult::A => unreachable!()
		}
		totgames += 1;
		games = if games == 9999 {
			println!("{} games in", totgames);
			0
		} else { games+1 };
	}
}
