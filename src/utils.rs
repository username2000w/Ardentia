#[derive(Debug, PartialEq, Eq, Default)]
pub enum MainMenuOption {
	#[default]
	NewGame,
	LoadGame,
	Quit,
}
