use ratatui::style::Color;

pub const DEFAULT_QUIZ_NAME: &str = "Quiz";
pub const LOBBY_LOCKED_MSG: &str = "The lobby is locked";
pub const PLAYER_NOT_IN_WAITING_LIST_MSG: &str = "Player not in waiting list";
pub const NICKNAME_ALREADY_TAKEN_MSG: &str = "Nickname already taken";
pub const DEFAULT_PORT: u16 = 8080;
pub const DEFAULT_GOODBYE_MESSAGE: &str = "Goodbye";
pub const MINIMAL_SCREEN_HEIGHT: u16 = 13;
pub const MINIMAL_ASCII_HEIGHT: u16 = 18;
pub const MINIMAL_QUESTION_HEIGHT: u16 = 30;
pub const MAXIMAL_CHOICE_LENGTH: usize = 200;
pub const MAXIMAL_QUESTION_LENGTH: usize = 200;
pub const MAXIMAL_CODE_LENGTH: usize = 400;
pub const MAXIMAL_NAME_LENGTH: usize = 20;
pub const PLAYER_KICKED_MESSAGE: &str = "You were kicked by the teacher";
pub const COLORS: [Color; 7] = [
    Color::Red,
    Color::Blue,
    Color::Green,
    Color::Yellow,
    Color::Magenta,
    Color::Cyan,
    Color::Gray,
];
