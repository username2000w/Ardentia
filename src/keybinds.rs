use std::ops::Index;

use ratatui::crossterm::event::{KeyCode, KeyEvent};

use crate::{
	app::App,
	room::WeaponUtils,
	screen::Screen,
	utils::{ChangeWeaponOption, CombatOption, MainMenuOption},
};

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
		if key.code == KeyCode::Enter
			&& !self
				.dungeon
				.rooms
				.index(self.dungeon.current_room)
				.monsters
				.is_empty()
		{
			self.switch_screen(Screen::CombatLoading);
		}
	}

	pub fn handle_combat(&mut self, key: KeyEvent) {
		let room = self.dungeon.get_current_room_mutable();
		let monsters = &mut room.monsters;
		let monster = &mut monsters[room.current_monster];
		let player = &mut self.player;

		match key.code {
			KeyCode::Enter => match self.current_combat_option {
				CombatOption::Attack => {
					if player.speed > monster.speed {
						player.attack(monster);
						if monster.is_alive() {
							monster.attack(player);
						}
					} else {
						if monster.is_alive() {
							monster.attack(player);
						}
						player.attack(monster);
					}

					if !monster.is_alive() {
						room.monster_slain();
						self.switch_screen(Screen::DefeatMonster);
					} else if player.is_dead() {
						self.switch_screen(Screen::DeadPlayer);
					}
				}
				CombatOption::Run => todo!("Run not implemented yet"),
			},
			KeyCode::Up => self.option_up(),
			KeyCode::Down => self.option_down(),
			_ => (),
		}
	}

	pub fn handle_change_weapon(&mut self, key: KeyEvent) {
		let room = self.dungeon.get_current_room_mutable();
		let player = &mut self.player;
		if room.treasures.contains_weapon() {
			let weapon = room.treasures.get_weapon().unwrap();
			match key.code {
				KeyCode::Enter => {
					match self.current_change_weapon_option {
						ChangeWeaponOption::Yes => player.equip(weapon),
						ChangeWeaponOption::No => (),
					}
					if room.is_empty() {
						let dungeon = &mut self.dungeon;
						if dungeon.is_there_rooms_left() {
							dungeon.next_room();
							self.switch_screen(Screen::RoomLoading);
						} else {
							self.switch_screen(Screen::MainMenu);
						}
					} else {
						self.switch_screen(Screen::CombatLoading);
					}
				}
				KeyCode::Up => self.option_up(),
				KeyCode::Down => self.option_down(),
				_ => (),
			}
		}
	}

	pub const fn option_down(&mut self) {
		match self.current_screen {
			Screen::MainMenu => match self.current_main_menu_option {
				MainMenuOption::NewGame => self.current_main_menu_option = MainMenuOption::LoadGame,
				_ => self.current_main_menu_option = MainMenuOption::Quit,
			},
			Screen::Combat => self.current_combat_option = CombatOption::Run,
			Screen::RoomResult => self.current_change_weapon_option = ChangeWeaponOption::No,
			_ => (),
		}
	}

	pub const fn option_up(&mut self) {
		match self.current_screen {
			Screen::MainMenu => match self.current_main_menu_option {
				MainMenuOption::Quit => self.current_main_menu_option = MainMenuOption::LoadGame,
				_ => self.current_main_menu_option = MainMenuOption::NewGame,
			},
			Screen::Combat => self.current_combat_option = CombatOption::Attack,
			Screen::RoomResult => self.current_change_weapon_option = ChangeWeaponOption::Yes,
			_ => (),
		}
	}
}
