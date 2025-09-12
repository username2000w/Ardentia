use std::ops::Index;

use ratatui::{
	layout::{Constraint, Layout},
	style::Stylize,
	text::Line,
	Frame,
};

use crate::{
	app::App,
	room::TreasureUtils,
	utils::{
		render_based_on_choice, render_centered, render_centered_bold_text, render_centered_solo,
		render_left_aligned_text_bold, render_list, render_list_centered,
		render_right_aligned_text_bold, render_title, CombatOption, MainMenuOption,
	},
};

#[derive(Debug, PartialEq, Eq, Default)]
pub enum Screen {
	#[default]
	MainMenu,
	DungeonLoading,
	RoomLoading,
	Room,
	RoomResult,
	Combat,
	CombatLoading,
	DefeatMonster,
	DeadPlayer,
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

		render_title(frame, "Project X", title_area);

		render_based_on_choice(
			frame,
			"New Game",
			new_game_area,
			&app.current_main_menu_option,
			&MainMenuOption::NewGame,
		);

		render_based_on_choice(
			frame,
			"Load Game",
			load_game_area,
			&app.current_main_menu_option,
			&MainMenuOption::LoadGame,
		);

		render_based_on_choice(
			frame,
			"Quit",
			exit_area,
			&app.current_main_menu_option,
			&MainMenuOption::Quit,
		);
	}

	pub fn dungeon_loading(frame: &mut Frame, app: &App) {
		render_centered_solo(
			frame,
			format!("You enter the level {} Dongeon.", app.dungeon.level),
		);
	}

	pub fn room_loading(frame: &mut Frame) {
		render_centered_solo(frame, "You enter a room.");
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

		render_title(frame, format!("Room : {}", room.name), title_area);

		render_centered_bold_text(frame, room.description.clone(), description_area);

		render_centered_bold_text(
			frame,
			format!("Difficulty : {}", room.difficulty),
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

		render_list(frame, monster_list, monsters_area);
		render_list(frame, treasure_list, treasure_area);
		render_centered(frame, "Enter", enter_area);
	}

	pub fn combat(frame: &mut Frame, app: &App) {
		let player = &app.player;
		let room = app.dungeon.rooms.index(app.current_room);
		let monster = room.monsters.index(room.current_monster);

		let [title_area, combat_area, answer_area] = Layout::vertical([
			Constraint::Length(3),
			Constraint::Fill(2),
			Constraint::Fill(1),
		])
		.areas(frame.area());
		let [player_stats_area, _, monster_stats_area] = Layout::horizontal([
			Constraint::Percentage(48),
			Constraint::Length(6),
			Constraint::Percentage(48),
		])
		.areas(combat_area);
		let [player_stats_name_area, player_stats_health_area, player_stats_attack_area, player_stats_defence_area, player_stats_speed_area] =
			Layout::vertical([
				Constraint::Length(1),
				Constraint::Length(1),
				Constraint::Length(1),
				Constraint::Length(1),
				Constraint::Length(1),
			])
			.areas(player_stats_area);
		let [monster_stats_name_area, monster_stats_health_area, monster_stats_attack_area, monster_stats_defence_area, monster_stats_speed_area] =
			Layout::vertical([
				Constraint::Length(1),
				Constraint::Length(1),
				Constraint::Length(1),
				Constraint::Length(1),
				Constraint::Length(1),
			])
			.areas(monster_stats_area);
		let [question_area, attack_button_area, run_button_area] = Layout::vertical([
			Constraint::Length(1),
			Constraint::Length(1),
			Constraint::Length(1),
		])
		.areas(answer_area);

		render_title(frame, "BATTLE", title_area);

		// Player

		render_right_aligned_text_bold(frame, player.name.clone(), player_stats_name_area);
		render_right_aligned_text_bold(
			frame,
			format!("Health : {}", player.health),
			player_stats_health_area,
		);
		render_right_aligned_text_bold(
			frame,
			format!("Attack : {}", player.attack),
			player_stats_attack_area,
		);
		render_right_aligned_text_bold(
			frame,
			format!("Defence : {}", player.defence),
			player_stats_defence_area,
		);
		render_right_aligned_text_bold(
			frame,
			format!("Speed : {}", player.speed),
			player_stats_speed_area,
		);

		// Monster
		render_left_aligned_text_bold(frame, monster.name.clone(), monster_stats_name_area);
		render_left_aligned_text_bold(
			frame,
			format!("Health : {}", monster.health),
			monster_stats_health_area,
		);
		render_left_aligned_text_bold(
			frame,
			format!("Attack : {}", monster.attack),
			monster_stats_attack_area,
		);
		render_left_aligned_text_bold(
			frame,
			format!("Defence : {}", monster.defence),
			monster_stats_defence_area,
		);
		render_left_aligned_text_bold(
			frame,
			format!("Speed : {}", monster.speed),
			monster_stats_speed_area,
		);

		// Question
		render_centered(frame, "What do you do ?", question_area);
		render_based_on_choice(
			frame,
			"Attack",
			attack_button_area,
			&app.current_combat_option,
			&CombatOption::Attack,
		);
		render_based_on_choice(
			frame,
			"Run",
			run_button_area,
			&app.current_combat_option,
			&CombatOption::Run,
		);
	}

	pub fn combat_loading(frame: &mut Frame, app: &App) {
		let room = app.dungeon.rooms.index(app.current_room);

		let monster = room.monsters.index(room.current_monster);

		render_centered_solo(
			frame,
			format!("A level {} {} appears !", monster.level, monster.name),
		);
	}

	pub fn room_result(frame: &mut Frame, app: &App) {
		let room = app.dungeon.rooms.index(app.current_room);

		let treasures = room.treasures.clone();

		let mut treasures_text = vec![];

		treasures_text.push(Line::from("Rewards:").red().bold());

		for treasure in treasures {
			if treasure.weapon.is_some() {
				treasures_text.push(
					Line::from(treasure.weapon.unwrap().to_string())
						.red()
						.bold(),
				);
			}
			if treasure.health_potion.is_some() {
				treasures_text.push(
					Line::from(treasure.health_potion.unwrap().to_string())
						.red()
						.bold(),
				);
			}
			if treasure.gold.is_some() {
				treasures_text.push(
					Line::from(format!("{} gold", treasure.gold.unwrap()))
						.red()
						.bold(),
				);
			}
		}

		#[allow(clippy::cast_possible_truncation)]
		let [area] = Layout::vertical([Constraint::Length(treasures_text.len() as u16)])
			.flex(ratatui::layout::Flex::Center)
			.areas(frame.area());

		render_list_centered(frame, treasures_text, area);
	}

	pub fn defeat_monster(frame: &mut Frame, app: &App) {
		let room = app.dungeon.rooms.index(app.current_room);

		let monster = room.monsters.index(room.current_monster);

		render_centered_solo(
			frame,
			format!("You defeated a level {} {} !", monster.level, monster.name),
		);
	}

	pub fn dead_player(frame: &mut Frame<'_>) {
		render_centered_solo(frame, "You are dead !");
	}
}
