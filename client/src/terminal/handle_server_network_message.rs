use crate::terminal::student::{StudentTerminal, StudentTerminalState};
use common::constants::NICKNAME_ALREADY_TAKEN_MSG;
use common::messages::network::CanJoin;
use common::messages::ServerNetworkMessage;
use common::terminal::terminal_actor::TerminalHandleServerNetworkMessage;

use common::terminal::widgets::choice::ChoiceSelectorState;

use log::debug;
use ratatui::widgets::{ListState, TableState};

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
                        self.state = StudentTerminalState::NameSelection {
                            name: self.name.clone(),
                            name_already_used: true,
                        };
                    } else {
                        self.state = StudentTerminalState::Error { message }
                    }
                    return Ok(());
                }
                self.state = StudentTerminalState::WaitingForGame {
                    list_state: ListState::default().with_selected(Some(0)),
                };
            }
            ServerNetworkMessage::NextQuestion(question) => {
                debug!("Student: handling next question");
                self.state = StudentTerminalState::Question {
                    question: question.clone(),
                    players_answered_count: 0,
                    answered: false,
                    start_time: chrono::Utc::now(),
                    duration_from_start: chrono::Duration::zero(),
                    choice_grid: question.question.into(),
                    choice_selector_state: ChoiceSelectorState::default(),
                };
            }
            ServerNetworkMessage::QuestionUpdate(update) => {
                debug!("Student: handling question update");
                let StudentTerminalState::Question {
                    question,
                    players_answered_count,
                    answered: _,
                    start_time: _,
                    duration_from_start: _,
                    choice_grid: _,
                    choice_selector_state: _,
                } = &mut self.state
                else {
                    anyhow::bail!("Terminal is not showing the question");
                };

                if question.question_index != update.question_index {
                    anyhow::bail!("Terminal is not showing the question with given index");
                }

                *players_answered_count = update.players_answered_count;
            }
            ServerNetworkMessage::QuestionEnded(question) => {
                debug!("Student: handling question ended");
                self.state = StudentTerminalState::Answers { answers: question };
            }
            ServerNetworkMessage::ShowLeaderboard(leaderboard) => {
                debug!("Student: handling show leaderboard");
                self.state = StudentTerminalState::Results {
                    results: leaderboard,
                    table_state: TableState::default().with_selected(Some(0)),
                };
            }
            ServerNetworkMessage::PlayersUpdate(update) => {
                debug!("Student: handling players update");
                self.players = update.players;
            }
            ServerNetworkMessage::TeacherDisconnected(_) => {
                debug!("Student: handling teacher disconnected");
                self.state = StudentTerminalState::Error {
                    message: "Teacher disconnected from the game".to_string(),
                };
            }
            _ => {}
        }
        Ok(())
    }
}
