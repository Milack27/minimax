use crate::*;

use ::minimax::{
    GameResult as MinimaxGameResult, GameState, Player as MinimaxPlayer, Status as MinimaxStatus,
};

impl From<Player> for MinimaxPlayer {
    fn from(player: Player) -> MinimaxPlayer {
        match player {
            Player::X => MinimaxPlayer::One,
            Player::O => MinimaxPlayer::Two,
        }
    }
}

impl From<MinimaxPlayer> for Player {
    fn from(player: MinimaxPlayer) -> Player {
        match player {
            MinimaxPlayer::One => Player::X,
            MinimaxPlayer::Two => Player::O,
        }
    }
}

impl From<GameResult> for MinimaxGameResult {
    fn from(result: GameResult) -> MinimaxGameResult {
        match result {
            GameResult::Draw => MinimaxGameResult::Draw,
            GameResult::Win(player) => MinimaxGameResult::Win(player.into()),
        }
    }
}

impl From<MinimaxGameResult> for GameResult {
    fn from(result: MinimaxGameResult) -> GameResult {
        match result {
            MinimaxGameResult::Draw => GameResult::Draw,
            MinimaxGameResult::Win(player) => GameResult::Win(player.into()),
        }
    }
}

impl From<Status> for MinimaxStatus {
    fn from(status: Status) -> MinimaxStatus {
        match status {
            Status::Running(player) => MinimaxStatus::Running(player.into()),
            Status::Finished(result) => MinimaxStatus::Finished(result.into()),
        }
    }
}

impl From<MinimaxStatus> for Status {
    fn from(status: MinimaxStatus) -> Status {
        match status {
            MinimaxStatus::Running(player) => Status::Running(player.into()),
            MinimaxStatus::Finished(result) => Status::Finished(result.into()),
        }
    }
}

impl GameState for TicTacToe {
    type Move = Place;
    type MoveError = MoveError;

    fn get_status(&self) -> MinimaxStatus {
        self.get_status().into()
    }

    fn possible_moves(&self) -> Vec<Place> {
        self.possible_moves()
    }

    fn make_move(&mut self, mov: &Place) -> Result<(), MoveError> {
        match self.status {
            Status::Finished(_) => Err(MoveError::InvalidStatus(self.status)),
            Status::Running(player) => self.make_move(player, *mov),
        }
    }
}
