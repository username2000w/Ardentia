use std::{
	ops::Index,
	process::exit,
	thread::{self, sleep},
	time::Duration,
};

use ratatui::{
	crossterm::event::{self, Event, KeyCode, KeyEventKind},
	DefaultTerminal, Frame,
};

use crate::{dungeon::Dungeon, entity::Player, screen::Screen, utils::MainMenuOption};
use color_eyre::Result;

#[derive(Debug, Default)]
pub struct App {
	pub current_screen: Screen,
	pub current_main_menu_option: MainMenuOption,
	pub dungeon: Dungeon,
	pub player: Player,
	pub current_room: usize,
}

impl App {
	pub fn run(&mut self, mut terminal: DefaultTerminal) -> Result<()> {
		loop {
			terminal.draw(|frame| self.draw(frame))?;
			if self.current_screen == Screen::RoomLoading {
				thread::sleep(Duration::from_secs(1));
				self.switch_screen(Screen::Room);
				continue;
			}
			if self.current_screen == Screen::DungeonLoading {
				thread::sleep(Duration::from_secs(1));
				self.switch_screen(Screen::RoomLoading);
				continue;
			}

			if let Event::Key(key) = event::read()? {
				let mut is_quitting = false;
				if key.kind == KeyEventKind::Press {
					if self.current_screen == Screen::MainMenu {
						is_quitting = self.handle_main_screen(key);
					}
					if self.current_screen == Screen::Room {
						self.handle_room(key);
					}
				}

				// Failsafe
				if key.kind == KeyEventKind::Press && key.code == KeyCode::Esc {
					break;
				}

				if is_quitting {
					break;
				}
			}
		}
		Ok(())
	}

	pub fn draw(&mut self, frame: &mut Frame) {
		match self.current_screen {
			Screen::MainMenu => Screen::main_menu(frame, self),
			Screen::DungeonLoading => Screen::dungeon_loading(frame, self),
			Screen::RoomLoading => Screen::room_loading(frame),
			Screen::RoomResult => todo!(),
			Screen::Combat => Screen::combat(frame, self),
			Screen::Room => Screen::room(frame, self),
		}
	}

	pub const fn option_down(&mut self) {
		match self.current_main_menu_option {
			MainMenuOption::NewGame => self.current_main_menu_option = MainMenuOption::LoadGame,
			_ => self.current_main_menu_option = MainMenuOption::Quit,
		}
	}

	pub const fn option_up(&mut self) {
		match self.current_main_menu_option {
			MainMenuOption::Quit => self.current_main_menu_option = MainMenuOption::LoadGame,
			_ => self.current_main_menu_option = MainMenuOption::NewGame,
		}
	}

	pub const fn switch_screen(&mut self, screen: Screen) {
		self.current_screen = screen;
	}

	pub fn create_player(&mut self) {
		self.player = Player::new("You");
	}

	pub fn create_dungeon(&mut self) {
		self.dungeon = Dungeon::new(1, 5);
	}
}
