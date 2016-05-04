extern crate rand;

use std::collections::HashMap;
use std::collections::hash_map::Entry;
use std::io::{self, Read};
use rand::Rng;

#[derive(Clone, Copy, Hash, PartialEq, Eq)]
enum Spot {
	O, X, A
}

#[derive(Clone, Copy, Hash, PartialEq, Eq)]
enum GameResult {
	O, X, A, OX
}

trait Player {
	fn mv(&mut self, [Spot; 9]) -> usize;
}

struct Ai<R: Rng> {
	rng: R,
	lut: HashMap<[Spot; 9], [(u32, u32); 9]>,
	choices: Vec<([Spot; 9], u8)>,
}

impl<R: Rng> Ai<R> {
	pub fn new(rng: R) -> Ai<R> {
		Ai {
			rng: rng,
			lut: Default::default(),
			choices: Default::default(),
		}
	}
	pub fn feedback(&mut self, won: bool) {
		for (key, choice) in self.choices.drain(0..) {
			let ent = self.lut.entry(key);
			match ent {
				Entry::Occupied(mut val) => {
					let mut newval = val.get_mut();
					if won {
						newval[choice as usize].0 += 1;
					} else {
						newval[choice as usize].1 += 1;
					}
				},
				Entry::Vacant(val) => {
					let mut newval: [(u32, u32); 9] = Default::default();
					if won {
						newval[choice as usize].0 = 1;
					} else {
						newval[choice as usize].1 = 1;
					}
					val.insert(newval);
				},
			}
		}
	}
}

struct Human;
impl Player for Human {
	fn mv(&mut self, b: [Spot; 9]) -> usize {
		prgame(b);
		let stdin = io::stdin();
		let mut line = String::new();
		stdin.read_line(&mut line);
		return match line.trim().parse::<usize>() {
			Ok(i) if i<9 => i,
			_ => self.mv(b),
		}
	}
}

fn sigmoid(x: f64) -> f64 {
	1.0 / (1.0 + (-x).exp())
}

fn wilson(up: f64, total: f64) -> f64 {
	/*let z = 2.326348;
	let z2 = z*z;
	let phat = up/total;
	(phat + z2/(2.0*total) - z*((phat*(1.0-phat)+z2/(4.0*total))/total).sqrt())/(1.0+z2/total)*/
	up - (total-up)
}

fn prgame(b: [Spot; 9]) -> () {
	for (i, &spot) in b.iter().enumerate() {
		print!("{}", match spot {
			Spot::O => "O",
			Spot::X => "X",
			Spot::A => "-",
		});
		if i%3 == 2 { println!("") }
	}
}

impl<R: Rng> Player for Ai<R> {
	fn mv(&mut self, b: [Spot; 9]) -> usize {
		let ent = self.lut.entry(b);
		let choice = match ent {
			Entry::Occupied(val) => {
				let mut max = -1.0/0.0;
				let mut maxi: Vec<usize> = Vec::new();
				for (i, &(win, loss)) in val.get().iter().enumerate() {
					let w = wilson(win as f64, (win+loss) as f64);
					if w > max {
						max = w;
						maxi.clear();
					}
					if w == max {
						maxi.push(i);
					}
				}
				println!("{:?} {} {:?}", maxi, max, val.get());
				*self.rng.choose(&maxi).unwrap_or(&0)
			},
			Entry::Vacant(val) => {
				val.insert([(0, 0); 9]);
				for (i, &spot) in b.iter().enumerate() {
					if spot == Spot::A { return i }
				}
				0
			}
		};
		self.choices.push((b, choice as u8));
		choice
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
		let mv = if player { p2.mv(game) } else { p1.mv(game) };
		if mv > 8 || game[mv] != Spot::A {
			prgame(game);
			return if player { GameResult::X } else { GameResult::O }
		}
		game[mv] = Spot::X;
		let winner = x_wins(&game);
		if winner != GameResult::A {
			//if prwin {
				prgame(game);
			//}
			return if winner == GameResult::X {
				if player { GameResult::O } else { GameResult::X }
			} else { winner }
		}
		flip_board(&mut game);
		player ^= true
	}
}

fn main() {	
	let mut ai1 = Ai::new(rand::XorShiftRng::new_unseeded());
	let mut ai2 = Ai::new(rand::XorShiftRng::new_unseeded());
	let mut first = false;
	let mut games: usize = 0;
	loop {
		if games == 0 {
			println!("{} begins", if first { 'O' } else { 'X' });
		}
		let winner = play(&mut ai1, &mut ai2, first, games == 0);
		match winner {
			GameResult::X => {
				ai1.feedback(true);
				ai1.feedback(true);
				ai2.feedback(false);
				ai2.feedback(false);
				if true || games == 0 { println!("X wins") }
			},
			GameResult::O => {
				ai2.feedback(true);
				ai2.feedback(true);
				ai1.feedback(false);
				ai1.feedback(false);
				if true || games == 0 { println!("O wins") }
			},
			GameResult::OX => {
				ai1.feedback(true);
				ai2.feedback(true);
				if true || games == 0 { println!("Draw") }
			},
			GameResult::A => unreachable!()
		}
		games = if games == 1000 { 0 } else { games+1 };
		first ^= true;
	}
}
