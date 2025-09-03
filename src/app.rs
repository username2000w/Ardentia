use std::{thread::sleep, time::Duration};

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
}

impl App {
	pub fn run(&mut self, mut terminal: DefaultTerminal) -> Result<()> {
		loop {
			terminal.draw(|frame| self.draw(frame))?;
			if self.current_screen == Screen::DungeonLoading {
				sleep(Duration::from_secs(2));
				self.switch_screen(Screen::Combat);
			}

			if let Event::Key(key) = event::read()? {
				if key.kind == KeyEventKind::Press {
					match key.code {
						KeyCode::Up => self.option_up(),
						KeyCode::Down => self.option_down(),
						KeyCode::Enter => match self.current_main_menu_option {
							MainMenuOption::NewGame => {
								self.create_player();
								self.create_dungeon();
								self.switch_screen(Screen::DungeonLoading);
							}
							MainMenuOption::LoadGame => break,
							MainMenuOption::Quit => break,
						},
						_ => (),
					}
				}
			}
		}
		Ok(())
	}

	pub fn draw(&mut self, frame: &mut Frame) {
		match self.current_screen {
			Screen::MainMenu => Screen::main_menu(frame, self),
			Screen::DungeonLoading => Screen::dungeon_loading(frame, self),
			Screen::RoomResult => todo!(),
			Screen::Combat => todo!(),
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

	fn create_player(&mut self) {
		self.player = Player::new("You");
	}

	fn create_dungeon(&mut self) {
		self.dungeon = Dungeon::new(1, 5);
	}
}
