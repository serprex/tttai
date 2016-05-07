extern crate rand;

use std::collections::HashMap;
use std::collections::hash_map::Entry;
use std::io;
use rand::{thread_rng, Rng, Rand, XorShiftRng};

mod engine;
use engine::*;

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

struct Ai2<R: Rng> {
	rng: R,
	lut: HashMap<[Spot; 9], [u32; 2]>,
	choices: Vec<[Spot; 9]>,
}

impl<R: Rng> Ai2<R> {
	pub fn new(rng: R) -> Self {
		Ai2 {
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

impl<R: Rng> Player for Ai2<R> {
	fn mv(&mut self, b: [Spot; 9]) -> u8 {
		let mut max = isize::min_value();
		let mut maxi: [u8; 9] = Default::default();
		let mut mxidx: usize = 0;
		for (i, &spot) in b.iter().enumerate() {
			if spot == Spot::A {
				let mut newb = b;
				newb[i] = Spot::X;
				let wl = *self.lut.get(&newb).unwrap_or(&[0, 0]);
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
		}
		*self.rng.choose(&maxi[..mxidx]).unwrap_or(&0)
	}

	fn feedback(&mut self, scale: i32) -> () {
		if scale == 0 {
			return self.choices.clear()
		}
		let cidx = if scale > 0 { 0 } else { 1 };
		let scale = scale.abs() as u32;
		for key in self.choices.drain(0..) {
			let ent = self.lut.entry(key);
			match ent {
				Entry::Occupied(mut val) => {
					let mut newval = val.get_mut();
					newval[cidx] += scale;
				},
				Entry::Vacant(val) => {
					let mut newval: [u32; 2] = Default::default();
					newval[cidx] = scale;
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

fn main() {	
	let mut trng = thread_rng();
	let mut rng = XorShiftRng::rand(&mut trng);
	let mut ai1 = Ai::new(XorShiftRng::rand(&mut trng));
	let mut ai2 = Ai2::new(XorShiftRng::rand(&mut trng));
	let mut ai3 = RngAi::new(XorShiftRng::rand(&mut trng));
	let mut games: usize = 0;
	let mut totgames: usize = 1;
	loop {
		let first = rng.gen::<bool>();
		let wentfirst = if first { 'O' } else { 'X' };
		let winner = if totgames%400000 == 0 {
			play(&mut ai2, &mut Human, first, false)
		} else if games&4 == 1 {
			play(&mut ai2, &mut ai3, first, games == 0)
		} else {
			play(&mut ai2, &mut ai1, first, games == 0)
		};
		match winner {
			GameResult::X => {
				ai1.feedback(2);
				ai2.feedback(-2);
				if games == 0 { println!("X wins, {}", wentfirst) }
			},
			GameResult::O => {
				ai2.feedback(-2);
				ai1.feedback(2);
				if games == 0 { println!("O wins, {}", wentfirst) }
			},
			GameResult::OX => {
				ai1.feedback(1);
				ai2.feedback(1);
				if games == 0 { println!("Draw, {}", wentfirst) }
			},
		}
		totgames += 1;
		games = if games == 9999 {
			println!("{} games in", totgames);
			0
		} else { games+1 };
	}
}
