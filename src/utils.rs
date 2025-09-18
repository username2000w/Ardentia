use ratatui::{
	layout::{Constraint, Flex, Layout, Rect},
	style::Stylize,
	text::{Line, Text},
	widgets::{Block, Paragraph, Wrap},
	Frame,
};

pub trait Choice {}

#[derive(Debug, PartialEq, Eq, Default)]
pub enum MainMenuOption {
	#[default]
	NewGame,
	LoadGame,
	Quit,
}

impl Choice for MainMenuOption {}

#[derive(Debug, PartialEq, Eq, Default)]
pub enum CombatOption {
	#[default]
	Attack,
	Run,
}

impl Choice for CombatOption {}

#[derive(Debug, PartialEq, Eq, Default)]
pub enum ChangeWeaponOption {
	#[default]
	Yes,
	No,
}

impl Choice for ChangeWeaponOption {}

pub fn render_title<'a, S: Into<String> + Into<Text<'a>>>(frame: &mut Frame, title: S, area: Rect) {
	let title: String = title.into();
	frame.render_widget(
		Paragraph::new(title.red().bold())
			.centered()
			.block(Block::bordered()),
		area,
	);
}

pub fn render_based_on_choice<'a, S: Into<String> + Into<Text<'a>>, C: Choice + PartialEq>(
	frame: &mut Frame,
	text: S,
	area: Rect,
	choice_current: &C,
	choice: &C,
) {
	let text: String = text.into();

	if choice_current == choice {
		frame.render_widget(
			Line::from(format!("> {text}").red().bold()).centered(),
			area,
		);
	} else {
		frame.render_widget(Line::from(text.red()).centered(), area);
	}
}

pub fn render_centered_solo<'a, S: Into<String> + Into<Text<'a>>>(frame: &mut Frame, text: S) {
	let text: String = text.into();
	let [text_area] = Layout::vertical([Constraint::Length(1)])
		.flex(Flex::Center)
		.areas(frame.area());

	frame.render_widget(Line::from(text.red()).centered(), text_area);
}

pub fn render_centered<'a, S: Into<String> + Into<Text<'a>>>(
	frame: &mut Frame,
	text: S,
	area: Rect,
) {
	let text: String = text.into();

	frame.render_widget(Line::from(text.red()).centered(), area);
}

pub fn render_centered_bold_text<'a, S: Into<String> + Into<Text<'a>>>(
	frame: &mut Frame,
	text: S,
	area: Rect,
) {
	let text: String = text.into();

	frame.render_widget(Text::from(Line::from(text.red().bold()).centered()), area);
}

pub fn render_list<'a, S: Into<String> + Into<Text<'a>>>(
	frame: &mut Frame,
	list: Vec<S>,
	area: Rect,
) where
	ratatui::prelude::Text<'a>: std::convert::From<std::vec::Vec<S>>,
{
	frame.render_widget(
		Paragraph::new(Text::from(list)).wrap(Wrap { trim: true }),
		area,
	);
}

pub fn render_list_centered<'a, S: Into<String> + Into<Text<'a>>>(
	frame: &mut Frame,
	list: Vec<S>,
	area: Rect,
) where
	ratatui::prelude::Text<'a>: std::convert::From<std::vec::Vec<S>>,
{
	frame.render_widget(
		Paragraph::new(Text::from(list))
			.wrap(Wrap { trim: true })
			.centered(),
		area,
	);
}

pub fn render_right_aligned_text_bold<'a, S: Into<String> + Into<Text<'a>>>(
	frame: &mut Frame,
	text: S,
	area: Rect,
) {
	let text: String = text.into();

	frame.render_widget(
		Paragraph::new(Line::from(text).red().bold()).right_aligned(),
		area,
	);
}

pub fn render_left_aligned_text_bold<'a, S: Into<String> + Into<Text<'a>>>(
	frame: &mut Frame,
	text: S,
	area: Rect,
) {
	let text: String = text.into();

	frame.render_widget(
		Paragraph::new(Line::from(text).red().bold()).left_aligned(),
		area,
	);
}
