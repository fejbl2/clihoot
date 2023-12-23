use crate::terminal::student::{StudentTerminal, StudentTerminalState};
use common::messages::network::CanJoin;
use common::messages::ServerNetworkMessage;
use common::terminal::terminal_actor::TerminalHandleServerNetworkMessage;

impl TerminalHandleServerNetworkMessage for StudentTerminal {
    fn handle_network_message(
        &mut self,
        network_message: ServerNetworkMessage,
    ) -> anyhow::Result<()> {
        match network_message {
            ServerNetworkMessage::JoinResponse(join) => {
                if let CanJoin::No(message) = join.can_join {
                    // TODO maybe rather return to the name selections screen
                    // and input the name and color again
                    self.state = StudentTerminalState::Error { message };
                    return Ok(());
                }
                self.state = StudentTerminalState::WaitingForGame {
                    players: join.players,
                };
            }
            ServerNetworkMessage::NextQuestion(question) => {
                self.state = StudentTerminalState::Question {
                    question,
                    players_answered_count: 0,
                    answered: false,
                };
            }
            ServerNetworkMessage::QuestionUpdate(update) => {
                let StudentTerminalState::Question {
                    question,
                    players_answered_count,
                    answered: _,
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
                if let StudentTerminalState::WaitingForGame { players } = &mut self.state {
                    *players = update.players;
                }
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
