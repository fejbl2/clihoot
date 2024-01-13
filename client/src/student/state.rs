use ratatui::widgets::{ListState, TableState};

use common::{
    messages::network::{NextQuestion, QuestionEnded, ShowLeaderboard},
    terminal::widgets::choice::{ChoiceGrid, ChoiceSelectorState},
};

#[derive(Debug)]
pub struct NameSelectionState {
    pub(super) name: String,
    pub(super) name_already_used: bool,
}

#[derive(Debug)]
pub struct ColorSelectionState {
    pub(super) list_state: ListState,
}

#[derive(Debug)]
pub struct WaitingForGameState {
    pub(super) list_state: ListState,
}

#[derive(Debug)]
pub struct QuestionState {
    pub(super) question: NextQuestion,
    pub(super) players_answered_count: usize,
    pub(super) answered: bool,
    pub(super) start_time: chrono::DateTime<chrono::Utc>,
    pub(super) duration_from_start: chrono::Duration,
    pub(super) choice_grid: ChoiceGrid,
    pub(super) choice_selector_state: ChoiceSelectorState,
    pub(super) multichoice_popup_visible: bool,
}

#[derive(Debug)]
pub struct AnswersState {
    pub(super) answers: QuestionEnded,
}

#[derive(Debug)]
pub struct ResultsState {
    pub(crate) results: ShowLeaderboard,
    pub(super) table_state: TableState,
}

#[derive(Debug)]
pub struct ErrorState {
    pub(super) message: String,
}

#[derive(Debug)]
pub enum StudentTerminalState {
    StartGame,
    NameSelection(NameSelectionState),
    ColorSelection(ColorSelectionState),
    WaitingForGame(WaitingForGameState),
    Question(QuestionState),
    Answers(AnswersState),
    Results(ResultsState),
    EndGame,
    Error(ErrorState),
}
