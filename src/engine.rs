pub trait Player {
	fn mv(&mut self, [Spot; 9]) -> u8;
	fn feedback(&mut self, i32) -> () {}
}

#[derive(Clone, Copy, Hash, PartialEq, Eq)]
pub enum Spot {
	O, X, A
}

#[derive(Clone, Copy, Hash, PartialEq, Eq)]
pub enum GameResult {
	O, X, OX
}

pub fn prgame(b: [Spot; 9]) -> () {
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

pub fn flip_board(b: &mut [Spot; 9]) -> () {
	for a in b.iter_mut() {
		*a = match *a {
			Spot::O => Spot::X,
			Spot::X => Spot::O,
			Spot::A => Spot::A,
		}
	}
}

pub fn x_wins(b: &[Spot; 9]) -> GameResult {
	match (b[0], b[1], b[2], b[3], b[4], b[5], b[6], b[7], b[8]) {
		(Spot::X, Spot::X, Spot::X, _, _, _, _, _, _) |
		(_, _, _, Spot::X, Spot::X, Spot::X, _, _, _) |
		(_, _, _, _, _, _, Spot::X, Spot::X, Spot::X) |
		(Spot::X, _, _, Spot::X, _, _, Spot::X, _, _) |
		(_, Spot::X, _, _, Spot::X, _, _, Spot::X, _) |
		(_, _, Spot::X, _, _, Spot::X, _, _, Spot::X) |
		(Spot::X, _, _, _, Spot::X, _, _, _, Spot::X) |
		(_, _, Spot::X, _, Spot::X, _, Spot::X, _, _) => GameResult::X,
		_ => if b.contains(&Spot::A) { GameResult::O } else { GameResult::OX },
	}
}

pub fn play<P1, P2>(p1: &mut P1, p2: &mut P2, mut player: bool, prwin: bool) -> GameResult
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
		if winner != GameResult::O {
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
