use std::io;
use std::collections::HashMap;
use std::collections::hash_map::Entry;
use rand::Rng;
use engine::*;

pub struct Ai<R: Rng> {
	rng: R,
	lut: HashMap<Game, [[u32; 2]; 9]>,
	choices: Vec<(Game, u8)>,
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

pub struct Ai2<R: Rng> {
	rng: R,
	lut: HashMap<Game, [u32; 2]>,
	choices: Vec<Game>,
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
		let ent = self.lut.entry(b);
		let choice = match ent {
			Entry::Occupied(val) => {
				let mut max = isize::min_value();
				let mut maxi: [u8; 9] = Default::default();
				let mut mxidx = 0;
				for (i, wl) in val.get().into_iter().enumerate() {
					if b.get(i as u8) != Spot::A { continue }
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
				for (i, spot) in b.into_iter().enumerate() {
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
	fn mv(&mut self, b: Game) -> u8 {
		let mut icand: [u8; 9] = Default::default();
		let mut icanlen: usize = 0;
		for (i, spot) in b.into_iter().enumerate() {
			if spot == Spot::A {
				icand[icanlen] = i as u8;
				icanlen += 1;
			}
		}
		*self.rng.choose(&icand[..icanlen]).unwrap_or(&0)
	}
}
