use std::f32;
use std::io;
use std::collections::HashMap;
use std::collections::hash_map::Entry;
use rand::Rng;
use engine::*;

pub struct Ai<R: Rng> {
	rng: R,
	lut: HashMap<Game, [u32; 2]>,
}

impl<R: Rng> Ai<R> {
	pub fn new(rng: R) -> Self {
		Ai {
			rng: rng,
			lut: Default::default(),
		}
	}
}

pub struct RngAi<R: Rng> {
	rng: R,
}

impl<R: Rng> RngAi<R> {
	pub fn new(rng: R) -> Self {
		RngAi {
			rng: rng,
		}
	}
}

pub struct NeuralAi<R: Rng> {
	rng: R,
	lut: [[[f32; 3]; 9]; 9],
}

impl<R: Rng> NeuralAi<R> {
	pub fn new(mut rng: R) -> Self {
		let initial_state = rng.gen::<[[[f32; 3]; 9]; 9]>();
		NeuralAi {
			rng: rng,
			lut: initial_state,
		}
	}
}

pub struct Human;
impl Player for Human {
	fn mv(&mut self, b: Game) -> u8 {
		b.prgame();
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
	fn mv(&mut self, b: Game) -> u8 {
		let mut max = isize::min_value();
		let mut maxi: [u8; 9] = Default::default();
		let mut mxidx: usize = 0;
		for (i, spot) in b.into_iter().enumerate() {
			let i = i as u8;
			if spot == Spot::A {
				let mut newb = b;
				newb.set(i, Spot::X);
				let wl = *self.lut.get(&newb).unwrap_or(&[0, 0]);
				let w = (wl[0] as isize) - (wl[1] as isize);
				if w > max {
					max = w;
					maxi[0] = i;
					mxidx = 1;
				} else if w+3 > max {
					maxi[mxidx] = i;
					mxidx += 1;
				}
			}
		}
		*self.rng.choose(&maxi[..mxidx]).unwrap_or(&0)
	}

	fn feedback(&mut self, mut good: bool, scale: u32, choices: &[Game]) -> () {
		for &key in choices.iter() {
			let ent = self.lut.entry(key);
			match ent {
				Entry::Occupied(mut val) => {
					let mut newval = val.get_mut();
					newval[if good { 0 } else { 1 }] += scale;
				},
				Entry::Vacant(val) => {
					let mut newval: [u32; 2] = Default::default();
					newval[if good { 0 } else { 1 }] = scale;
					val.insert(newval);
				},
			}
			good ^= true
		}
	}
}

impl<R: Rng> Player for RngAi<R> {
	fn mv(&mut self, b: Game) -> u8 {
		let mut icand: [u8; 9] = Default::default();
		let mut icanlen: usize = 0;
		for (i, spot) in b.into_iter().enumerate() {
			let i = i as u8;
			if spot == Spot::A {
				let mut testb = b;
				testb.set(i, Spot::X);
				if testb.x_wins() == GameResult::X {
					return i
				}
				icand[icanlen] = i;
				icanlen += 1;
			}
		}
		*self.rng.choose(&icand[..icanlen]).unwrap_or(&0)
	}
}

impl<R: Rng> Player for NeuralAi<R> {
	fn mv(&mut self, b: Game) -> u8 {
		let mut votes: [f32; 9] = Default::default();
		for (i, spot) in b.into_iter().enumerate() {
			let spoti = match spot {
				Spot::X => 0,
				Spot::O => 1,
				Spot::A => 2,
			};
			for j in 0..9 {
				votes[j] += self.lut[i][j][if j == i { 0 } else { spoti }];
			}
		}
		let mut max: [u8; 9] = Default::default();
		let mut maxv: f32 = f32::NEG_INFINITY;
		let mut mxlen: usize = 0;
		for (ius, &vote) in votes.into_iter().enumerate() {
			let i = ius as u8;
			if b.get(i) == Spot::A {
				if vote > maxv {
					maxv = vote;
					max[0] = i;
					mxlen = 1;
				} else if vote == maxv {
					max[mxlen] = i;
					mxlen += 1;
				}
			}
		}
		*self.rng.choose(&max[..mxlen]).unwrap_or(&0)
	}

	fn feedback(&mut self, mut good: bool, scale: u32, choices: &[Game]) -> () {
		for &key in choices.iter() {
			for (i, spot) in key.into_iter().enumerate() {
				let spoti = match spot {
					Spot::X => 0,
					Spot::O => 1,
					Spot::A => 2,
				};
				for j in 0..9 {
					self.lut[i][j][spoti] += scale as f32 / if good { 16.0 } else { -16.0 };
				}
			}
			good ^= true
		}
	}
}
