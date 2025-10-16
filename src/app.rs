use std::{
    thread::{self},
    time::Duration,
};

use ratatui::{
    crossterm::event::{self, Event, KeyCode, KeyEventKind},
    DefaultTerminal, Frame,
};

use crate::{
    dungeon::Dungeon,
    entity::Player,
    screen::Screen,
    utils::{ChangeWeaponOption, CombatOption, MainMenuOption},
    zones::zone::Zone,
};
use color_eyre::Result;

#[allow(missing_debug_implementations)]
#[derive(Default)]
pub struct App {
    pub current_screen: Screen,
    pub current_main_menu_option: MainMenuOption,
    pub current_combat_option: CombatOption,
    pub current_change_weapon_option: ChangeWeaponOption,

    pub player: Player,
    pub dungeon: Dungeon,
}

impl App {
    pub fn run(&mut self, mut terminal: DefaultTerminal) -> Result<()> {
        loop {
            terminal.draw(|frame| self.draw(frame))?;

            let has_waited_list = [
                self.wait(&Screen::RoomLoading, 1, Screen::Room),
                self.wait(&Screen::DungeonLoading, 1, Screen::RoomLoading),
                self.wait(&Screen::CombatLoading, 1, Screen::Combat),
                self.wait(&Screen::DefeatMonster, 1, Screen::RoomResult),
                self.wait(&Screen::DeadPlayer, 5, Screen::MainMenu),
                self.wait(&Screen::RunScreen, 1, Screen::MainMenu),
            ];

            if has_waited_list.contains(&true) {
                continue;
            }

            if let Event::Key(key) = event::read()? {
                let mut is_quitting = false;
                if key.kind == KeyEventKind::Press {
                    match self.current_screen {
                        Screen::MainMenu => is_quitting = self.handle_main_screen(key),
                        Screen::Room => self.handle_room(key),
                        Screen::Combat => self.handle_combat(key),
                        Screen::RoomResult => self.handle_change_weapon(key),
                        _ => (),
                    }

                    // Failsafe
                    if key.code == KeyCode::Esc {
                        break;
                    }
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
            Screen::RoomResult => Screen::room_result(frame, self),
            Screen::Combat => Screen::combat(frame, self),
            Screen::Room => Screen::room(frame, self),
            Screen::CombatLoading => Screen::combat_loading(frame, self),
            Screen::DefeatMonster => Screen::defeat_monster(frame, self),
            Screen::DeadPlayer => Screen::dead_player(frame),
            Screen::RunScreen => Screen::run_screen(frame),
        }
    }

    pub const fn switch_screen(&mut self, screen: Screen) {
        self.current_screen = screen;
    }

    pub fn create_player(&mut self) {
        self.player = Player::new("You");
    }

    #[allow(clippy::missing_panics_doc)]
    pub fn create_dungeon(&mut self) {
        let zone = Zone::get_available_zones()
            .first()
            .expect("Zone list should not be empty")
            .clone();

        self.dungeon = Dungeon::new(zone);
        self.dungeon.start();
    }

    fn wait(&mut self, room_to_wait: &Screen, secs: u64, room_to_switch_to: Screen) -> bool {
        if &self.current_screen == room_to_wait {
            thread::sleep(Duration::from_secs(secs));
            self.switch_screen(room_to_switch_to);
            true
        } else {
            false
        }
    }
}
