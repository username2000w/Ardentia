use std::ops::Index;

use ratatui::crossterm::event::{KeyCode, KeyEvent};

use crate::{app::App, screen::Screen, utils::MainMenuOption};

impl App {
	pub fn handle_main_screen(&mut self, key: KeyEvent) -> bool {
		match key.code {
			KeyCode::Up => self.option_up(),
			KeyCode::Down => self.option_down(),
			KeyCode::Enter => match self.current_main_menu_option {
				MainMenuOption::NewGame => {
					self.create_player();
					self.create_dungeon();
					self.switch_screen(Screen::DungeonLoading);
				}
				MainMenuOption::LoadGame => todo!(),
				MainMenuOption::Quit => return true,
			},
			_ => (),
		}
		false
	}

	pub fn handle_room(&mut self, key: KeyEvent) {
		if self.current_screen == Screen::Room
			&& key.code == KeyCode::Enter
			&& !self
				.dungeon
				.rooms
				.index(self.current_room)
				.monsters
				.is_empty()
		{
			self.switch_screen(Screen::Combat);
		}
	}
}
