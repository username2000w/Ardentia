use std::ops::Index;

use ratatui::{
	layout::{Constraint, Flex, Layout},
	style::Stylize,
	text::Line,
	widgets::{Block, Paragraph},
	Frame,
};

use crate::{app::App, utils::MainMenuOption};

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
		let treasure_number = (room.treasure.len() + 1) as u16;
		let areas = Layout::vertical([
			Constraint::Length(3),
			Constraint::Length(4),
			Constraint::Length(1),
			Constraint::Length(1),
			Constraint::Length(monster_number),
			Constraint::Length(treasure_number),
		]);

		let [title_area, _, description_area, difficulty_area, monsters_area, treasure_area] =
			areas.areas(frame.area());

		let title_text = format!("Room : {}", room.name);
		let title = Line::from(title_text.red().bold()).centered();

		frame.render_widget(Paragraph::new(title).block(Block::bordered()), title_area);

		let difficulty_text = format!("Room : {}", room.name);
		let difficulty = Line::from(difficulty_text.red().bold()).centered();
		frame.render_widget(
			Paragraph::new(difficulty).block(Block::bordered()),
			difficulty_area,
		);

		let mut monster_list = Line::default();

		if !room.monsters.is_empty() {
			monster_list.push_span("Monsters :");

			for monster in room.monsters.clone() {
				monster_list.push_span(format!("{}", monster.name));
			}
		}
	}
}
