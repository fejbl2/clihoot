use log::debug;
use ratatui::widgets::{ListState, TableState};

use common::{
    constants::NICKNAME_ALREADY_TAKEN_MSG,
    messages::{network::CanJoin, ServerNetworkMessage},
    terminal::{
        terminal_actor::TerminalHandleServerNetworkMessage, widgets::choice::ChoiceSelectorState,
    },
};

use crate::{
    music_actor::{MusicMessage, SoundEffectMessage},
    student::{
        state::{
            AnswersState, ErrorState, NameSelectionState, QuestionState, ResultsState,
            StudentTerminalState, WaitingForGameState,
        },
        terminal::StudentTerminal,
    },
};

impl TerminalHandleServerNetworkMessage for StudentTerminal {
    fn handle_network_message(
        &mut self,
        network_message: ServerNetworkMessage,
    ) -> anyhow::Result<()> {
        match network_message {
            ServerNetworkMessage::JoinResponse(join) => {
                debug!("Student: handling join response");
                self.players = join.players;
                if let CanJoin::No(message) = join.can_join {
                    if message == NICKNAME_ALREADY_TAKEN_MSG {
                        self.state = StudentTerminalState::NameSelection(NameSelectionState {
                            name: self.name.clone(),
                            name_already_used: true,
                        });
                    } else {
                        self.state = StudentTerminalState::Error(ErrorState { message })
                    }
                    return Ok(());
                }
                self.state = StudentTerminalState::WaitingForGame(WaitingForGameState {
                    list_state: ListState::default().with_selected(Some(0)),
                });
            }
            ServerNetworkMessage::NextQuestion(question) => {
                debug!("Student: handling next question");
                self.music_address.do_send(MusicMessage::Countdown);
                self.state = StudentTerminalState::Question(QuestionState {
                    question: question.clone(),
                    players_answered_count: 0,
                    answered: false,
                    start_time: chrono::Utc::now(),
                    duration_from_start: chrono::Duration::zero(),
                    choice_grid: question.question.into(),
                    choice_selector_state: ChoiceSelectorState::default(),
                });
            }
            ServerNetworkMessage::QuestionUpdate(update) => {
                debug!("Student: handling question update");
                let StudentTerminalState::Question(state) = &mut self.state else {
                    anyhow::bail!("Terminal is not showing the question");
                };

                if state.question.question_index != update.question_index {
                    anyhow::bail!("Terminal is not showing the question with given index");
                }

                state.players_answered_count = update.players_answered_count;
            }
            ServerNetworkMessage::QuestionEnded(question) => {
                debug!("Student: handling question ended");
                self.music_address.do_send(SoundEffectMessage::Gong);
                self.music_address.do_send(MusicMessage::NoMusic);
                self.state = StudentTerminalState::Answers(AnswersState { answers: question });
            }
            ServerNetworkMessage::ShowLeaderboard(leaderboard) => {
                debug!("Student: handling show leaderboard");
                self.state = StudentTerminalState::Results(ResultsState {
                    results: leaderboard,
                    table_state: TableState::default().with_selected(Some(0)),
                });
            }
            ServerNetworkMessage::PlayersUpdate(update) => {
                debug!("Student: handling players update");
                self.players = update.players;
            }
            ServerNetworkMessage::TeacherDisconnected(_) => {
                debug!("Student: handling teacher disconnected");
                self.state = StudentTerminalState::Error(ErrorState {
                    message: "Teacher disconnected from the game".to_string(),
                });
            }
            _ => {}
        }
        Ok(())
    }
}
