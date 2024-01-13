use ratatui::widgets::{ListState, TableState};

use common::messages::network::{NextQuestion, QuestionEnded, ShowLeaderboard};

#[derive(Debug)]
pub struct WaitingForGameState {
    pub(super) list_state: ListState,
    pub(super) kick_popup_visible: bool,
}

#[derive(Debug)]
pub struct QuestionState {
    pub(super) question: NextQuestion,
    pub(super) players_answered_count: usize,
    pub(super) start_time: chrono::DateTime<chrono::Utc>,
    pub(super) duration_from_start: chrono::Duration,
    pub(super) skip_popup_visible: bool,
}

#[derive(Debug)]
pub struct AnswersState {
    pub(super) answers: QuestionEnded,
}

#[derive(Debug)]
pub struct ResultsState {
    pub(super) results: ShowLeaderboard,
    pub(super) table_state: TableState,
    pub(super) kick_popup_visible: bool,
}

#[derive(Debug)]
pub struct ErrorState {
    pub(super) message: String,
}

#[derive(Debug)]
pub enum TeacherTerminalState {
    StartGame,
    WaitingForGame(WaitingForGameState),
    Question(QuestionState),
    Answers(AnswersState),
    Results(ResultsState),
    EndGame,
    Error(ErrorState),
}
