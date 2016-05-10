extern crate rand;
mod engine;
mod ai;

use rand::{XorShiftRng, Rand, Rng};
use engine::play;

fn main() {	
	let mut trng = rand::thread_rng();
	let mut rng = XorShiftRng::rand(&mut trng);
	let mut ai1 = ai::Ai::new(XorShiftRng::rand(&mut trng));
	let mut ai2 = ai::NeuralAi::new(XorShiftRng::rand(&mut trng));
	let mut ai3 = ai::RngAi::new(XorShiftRng::rand(&mut trng));
	let mut games: usize = 0;
	let mut totgames: usize = 1;
	loop {
		let first = rng.gen::<bool>();
		if totgames%400000 == 0 {
			play(&mut ai2, &mut ai::Human, first, false)
		} else if games&4 == 1 {
			play(&mut ai2, &mut ai3, first, games == 0)
		} else {
			play(&mut ai2, &mut ai1, first, games == 0)
		}
		totgames += 1;
		games = if games == 9999 {
			println!("{} games in", totgames);
			0
		} else { games+1 };
	}
}
