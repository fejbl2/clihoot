use crate::terminal::student::{StudentTerminal, StudentTerminalState};
use common::model::ServerNetworkMessage;
use common::terminal::terminal_actor::TerminalHandleServerNetworkMessage;

impl TerminalHandleServerNetworkMessage for StudentTerminal {
    fn handle_network_message(
        &mut self,
        network_message: ServerNetworkMessage,
    ) -> anyhow::Result<()> {
        match network_message {
            ServerNetworkMessage::NextQuestion(question) => {
                self.state = StudentTerminalState::Question {
                    question,
                    players_answered_count: 0,
                    answered: false,
                };
                Ok(())
            }
            ServerNetworkMessage::QuestionUpdate(update) => {
                let StudentTerminalState::Question {
                    question,
                    players_answered_count,
                    answered: _,
                } = &mut self.state
                else {
                    anyhow::bail!("foo");
                };

                if question.question_index != update.question_index {
                    anyhow::bail!("bar");
                }

                *players_answered_count = update.players_answered_count;

                Ok(())
            }
            ServerNetworkMessage::QuestionEnded(question) => {
                self.state = StudentTerminalState::Answers { answer: question };
                Ok(())
            }
            ServerNetworkMessage::ShowLeaderboard(leaderboard) => {
                self.state = StudentTerminalState::Results {
                    results: leaderboard,
                };
                Ok(())
            }
            ServerNetworkMessage::PlayersUpdate(update) => {
                // TODO check that we are waiting in the lobby
                // TODO update the players
                Ok(())
            }
            // TODO join response/tryjoinresponse
            _ => Ok(()),
        }
    }
}
