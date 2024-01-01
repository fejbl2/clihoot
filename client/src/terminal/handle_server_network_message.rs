use crate::terminal::student::{StudentTerminal, StudentTerminalState};
use common::constants::NICKNAME_ALREADY_TAKEN_MSG;
use common::messages::network::CanJoin;
use common::messages::ServerNetworkMessage;
use common::terminal::terminal_actor::TerminalHandleServerNetworkMessage;
use ratatui::widgets::ListState;

impl TerminalHandleServerNetworkMessage for StudentTerminal {
    fn handle_network_message(
        &mut self,
        network_message: ServerNetworkMessage,
    ) -> anyhow::Result<()> {
        match network_message {
            ServerNetworkMessage::JoinResponse(join) => {
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
                self.state = StudentTerminalState::Question {
                    question: question.clone(),
                    players_answered_count: 0,
                    answered: false,
                    choice_selector_state: question.question.into(),
                };
            }
            ServerNetworkMessage::QuestionUpdate(update) => {
                let StudentTerminalState::Question {
                    question,
                    players_answered_count,
                    answered: _,
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
                self.state = StudentTerminalState::Answers { answers: question };
            }
            ServerNetworkMessage::ShowLeaderboard(leaderboard) => {
                self.state = StudentTerminalState::Results {
                    results: leaderboard,
                };
            }
            ServerNetworkMessage::PlayersUpdate(update) => {
                self.players = update.players;
            }
            ServerNetworkMessage::TeacherDisconnected(_) => {
                self.state = StudentTerminalState::Error {
                    message: "Teacher disconnected from the game".to_string(),
                };
            }
            _ => {}
        }
        Ok(())
    }
}
