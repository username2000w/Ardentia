use std::{io, process::exit};

use project_x::{dungeon::Dungeon, entity::Player};

fn main() {
	loop {
		print!("\x1bc\x1b[1;1H");

		println!("==============================================");
		println!(" *--*-            PROJECT X            *-*--* ");
		println!("==============================================\n");

		println!("Welcome adventurer!");
		println!("ARDENTIA - A Dungeon Crawler Game");

		println!("1. New Game");
		println!("2. Load Game (WIP) 🤡🤡🤡");
		println!("3. Exit\n");

		let mut input = String::new();

		io::stdin()
			.read_line(&mut input)
			.expect("error: unable to read user input");

		match input.trim() {
			"1" => {
				print!("\x1bc\x1b[1;1H");
				let mut player = Player::new(String::from("You"));
				let mut dungeon = Dungeon::new(1, 3);
				dungeon.enter(&mut player);
			}
			"2" => println!("Load Game!"),
			"3" => {
				print!("\x1bc\x1b[1;1H");
				exit(0);
			}
			_ => println!("Invalid input!"),
		}
	}
}
