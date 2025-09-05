use std::ops::Index;

use ratatui::{
	layout::{Constraint, Flex, Layout},
	style::Stylize,
	text::{Line, Text},
	widgets::{Block, Paragraph, Wrap},
	Frame,
};

use crate::{app::App, room::TreasureUtils, utils::MainMenuOption};

#[derive(Debug, PartialEq, Eq, Default)]
pub enum Screen {
	#[default]
	MainMenu,
	DungeonLoading,
	RoomLoading,
	Room,
	RoomResult,
	Combat,
}

impl Screen {
	pub fn main_menu(frame: &mut Frame, app: &App) {
		let areas = Layout::vertical([
			Constraint::Length(3),
			Constraint::Length(4),
			Constraint::Length(1),
			Constraint::Length(1),
			Constraint::Length(1),
		]);

		let [title_area, _, new_game_area, load_game_area, exit_area] = areas.areas(frame.area());

		let title = Line::from("Project X".red().bold()).centered();

		frame.render_widget(Paragraph::new(title).block(Block::bordered()), title_area);

		if app.current_main_menu_option == MainMenuOption::NewGame {
			frame.render_widget(
				Line::from("> New Game".red().bold()).centered(),
				new_game_area,
			);
		} else {
			frame.render_widget(Line::from("New Game".red()).centered(), new_game_area);
		}

		if app.current_main_menu_option == MainMenuOption::LoadGame {
			frame.render_widget(
				Line::from("> Load Game".red().bold()).centered(),
				load_game_area,
			);
		} else {
			frame.render_widget(Line::from("Load Game".red()).centered(), load_game_area);
		}

		if app.current_main_menu_option == MainMenuOption::Quit {
			frame.render_widget(Line::from("> Quit".red().bold()).centered(), exit_area);
		} else {
			frame.render_widget(Line::from("Quit".red()).centered(), exit_area);
		}
	}

	pub fn dungeon_loading(frame: &mut Frame, app: &App) {
		let [text_area] = Layout::vertical([Constraint::Length(1)])
			.flex(Flex::Center)
			.areas(frame.area());

		let text = format!("You enter the level {} Dongeon.", app.dungeon.level);

		frame.render_widget(Line::from(text.red()).centered(), text_area);
	}

	pub fn room_loading(frame: &mut Frame) {
		let [text_area] = Layout::vertical([Constraint::Length(1)])
			.flex(Flex::Center)
			.areas(frame.area());

		frame.render_widget(Line::from("You enter a room.".red()).centered(), text_area);
	}

	pub fn room(frame: &mut Frame, app: &App) {
		let room = app.dungeon.rooms.index(app.current_room);
		let monster_number = (room.monsters.len() + 1) as u16;
		let treasure_number = room.treasures.treasure_len() + 1;
		let areas = Layout::vertical([
			Constraint::Length(3),               // Title
			Constraint::Length(4),               // Space
			Constraint::Length(1),               // Descritpion
			Constraint::Length(1),               // Space
			Constraint::Length(1),               // Difficulty
			Constraint::Length(1),               // Space
			Constraint::Length(monster_number),  // Monsters
			Constraint::Length(1),               // Space
			Constraint::Length(treasure_number), // Treasures
			Constraint::Length(5),               // Space
			Constraint::Length(5),               // Enter
		]);

		let [title_area, _, description_area, _, difficulty_area, _, monsters_area, _, treasure_area, _, enter_area] =
			areas.areas(frame.area());

		let title_text = format!("Room : {}", room.name);
		let title = Line::from(title_text.red().bold()).centered();

		frame.render_widget(
			Text::from(Line::from(room.description.clone().red().bold()).centered()),
			description_area,
		);

		frame.render_widget(Paragraph::new(title).block(Block::bordered()), title_area);

		let difficulty_text = format!("Difficulty : {}", room.difficulty);
		frame.render_widget(
			Line::from(difficulty_text.red().bold()).centered(),
			difficulty_area,
		);

		let mut monster_list = vec![];

		if room.monsters.is_empty() {
			monster_list.push(Line::from("No monsters. Neat!".red().bold()).centered());
		} else {
			monster_list.push(Line::from("Monsters :".red().bold()).centered());

			for monster in room.monsters.clone() {
				monster_list.push(
					Line::from(format!("{} - Level {}", monster.name, monster.level).red())
						.centered(),
				);
			}
		}

		let mut treasure_list = vec![];

		if room.treasures.is_empty() {
			treasure_list.push(Line::from("No treasures. Sad!".red().bold()).centered());
		} else {
			treasure_list.push(Line::from("Treasures :".red().bold()).centered());

			for treasure in room.treasures.clone() {
				if let Some(weapon) = treasure.weapon {
					treasure_list
						.push(Line::from(format!("A {} weapon", weapon.rarity).red()).centered());
				}

				if let Some(gold) = treasure.gold {
					treasure_list.push(Line::from(format!("{gold} gold").red()).centered());
				}
			}
		}

		frame.render_widget(
			Paragraph::new(Text::from(monster_list)).wrap(Wrap { trim: true }),
			monsters_area,
		);

		frame.render_widget(
			Paragraph::new(Text::from(treasure_list)).wrap(Wrap { trim: true }),
			treasure_area,
		);

		frame.render_widget(
			Paragraph::new(Text::from("Enter").bold().red()).centered(),
			enter_area,
		);
	}

	pub fn combat(frame: &mut Frame, app: &App) {}
}
