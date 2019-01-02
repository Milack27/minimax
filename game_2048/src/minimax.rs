use crate::*;

use std::convert::TryFrom;

use ::minimax::{
    Player as MinimaxPlayer,
    GameResult as MinimaxGameResult,
    Status as MinimaxStatus,
    GameState,
};

impl From<Player> for MinimaxPlayer {
    fn from(player: Player) -> MinimaxPlayer {
        match player {
            Player::Human => MinimaxPlayer::One,
            Player::Robot => MinimaxPlayer::Two, 
        }
    }
}

impl From<MinimaxPlayer> for Player {
    fn from(player: MinimaxPlayer) -> Player {
        match player {
            MinimaxPlayer::One => Player::Human,
            MinimaxPlayer::Two => Player::Robot, 
        }
    }
}

impl From<Status> for MinimaxStatus {
    fn from(status: Status) -> MinimaxStatus {
        match status {
            Status::Running(player) => MinimaxStatus::Running(player.into()),
            Status::Finished => MinimaxStatus::Finished(MinimaxGameResult::Win(Player::Robot.into())), 
        }
    }
}

impl TryFrom<MinimaxStatus> for Status {
    type Error = ();

    fn try_from(status: MinimaxStatus) -> Result<Status, ()> {
        match status {
            MinimaxStatus::Running(player) => Ok(Status::Running(player.into())),
            MinimaxStatus::Finished(MinimaxGameResult::Win(MinimaxPlayer::Two)) => Ok(Status::Finished),
            _ => Err(())
        }
    }
}

impl GameState for Game2048 {
    type Move = Move;
    type MoveError = MoveError;

    fn get_status(&self) -> MinimaxStatus {
        self.get_status().into()
    }

    fn possible_moves(&self) -> Vec<Move> {
        self.possible_moves()
    }

    fn make_move(&mut self, mov: &Move) -> Result<(), MoveError> {
        self.make_move(*mov)
    }
}