use std::fmt::format;

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
}
