use actix::{Context, Handler};
use anyhow::bail;

use crate::{
    lobby::state::{Lobby, Phase},
    messages::lobby::SwitchToLeaderboard,
};

impl Handler<SwitchToLeaderboard> for Lobby {
    type Result = anyhow::Result<()>;

    fn handle(&mut self, _msg: SwitchToLeaderboard, _: &mut Context<Self>) -> Self::Result {
        // have to be in phase `AfterQuestion`
        let Phase::AfterQuestion(index) = self.phase else {
            bail!("Cannot switch to leaderboard, not in AfterQuestion phase");
        };

        // send the leaderboard to all clients
        let is_final = self.send_leaderboard(index)?;

        // set the phase to ShowingLeaderboard or GameEnded (if final round)
        self.phase = if is_final {
            Phase::GameEnded
        } else {
            Phase::ShowingLeaderboard(index)
        };

        Ok(())
    }
}
