use std::ops::Index;

mod display;
mod minimax;

#[cfg(test)]
mod test;

const GRID_SIZE: usize = 9;

#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq)]
pub enum Place {
    UpperLeft,
    Upper,
    UpperRight,
    Left,
    Center,
    Right,
    LowerLeft,
    Lower,
    LowerRight,
}

#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq)]
pub enum Player {
    X,
    O,
}

#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq)]
pub enum GameResult {
    Draw,
    Win(Player),
}

#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq)]
pub enum Status {
    Running(Player),
    Finished(GameResult),
}

#[derive(Clone, Debug, Hash, PartialEq, Eq)]
pub struct TicTacToe {
    status: Status,
    grid: [Option<Player>; GRID_SIZE],
}

#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq)]
pub enum MoveError {
    InvalidStatus(Status),
    WrongPlayer(Player),
    PlaceAlreadyUsed(Place, Player),
}

impl Player {
    fn other(self) -> Player {
        match self {
            Player::X => Player::O,
            Player::O => Player::X,
        }
    }
}

impl From<GameResult> for Option<Player> {
    fn from(result: GameResult) -> Option<Player> {
        match result {
            GameResult::Draw => None,
            GameResult::Win(player) => Some(player),
        }
    }
}

impl From<Option<Player>> for GameResult {
    fn from(winner: Option<Player>) -> GameResult {
        match winner {
            None => GameResult::Draw,
            Some(player) => GameResult::Win(player),
        }
    }
}

impl GameResult {
    pub fn winner(self) -> Option<Player> {
        self.into()
    }
}

impl TicTacToe {
    pub fn new() -> TicTacToe {
        TicTacToe {
            status: Status::Running(Player::X),
            grid: [None; GRID_SIZE],
        }
    }

    pub fn get_status(&self) -> Status {
        self.status
    }

    fn get_grid_index(place: Place) -> usize {
        match place {
            Place::UpperLeft => 0,
            Place::Upper => 1,
            Place::UpperRight => 2,
            Place::Left => 3,
            Place::Center => 4,
            Place::Right => 5,
            Place::LowerLeft => 6,
            Place::Lower => 7,
            Place::LowerRight => 8,
        }
    }

    fn get_grid_place(index: usize) -> Place {
        match index {
            0 => Place::UpperLeft,
            1 => Place::Upper,
            2 => Place::UpperRight,
            3 => Place::Left,
            4 => Place::Center,
            5 => Place::Right,
            6 => Place::LowerLeft,
            7 => Place::Lower,
            8 => Place::LowerRight,
            _ => panic!("{} is not a valid index for a place in the grid.", index),
        }
    }

    pub fn get_place(&self, place: Place) -> Option<Player> {
        self[place]
    }

    fn set_place(&mut self, place: Place, player: Option<Player>) {
        self.grid[TicTacToe::get_grid_index(place)] = player;
    }

    pub fn possible_moves(&self) -> Vec<Place> {
        match self.status {
            Status::Running(_) => self
                .grid
                .iter()
                .cloned()
                .enumerate()
                .filter_map(|(i, p)| match p {
                    None => Some(TicTacToe::get_grid_place(i)),
                    Some(_) => None,
                })
                .collect(),
            Status::Finished(_) => Vec::new(),
        }
    }

    fn check_triple(&self, places: [Place; 3]) -> Option<Player> {
        let player = self[places[0]];

        if places.iter().map(|&p| self[p]).all(|p| p == player) {
            player
        } else {
            None
        }
    }

    fn check_win(&self) -> Option<GameResult> {
        let triples = [
            [Place::UpperLeft, Place::Upper, Place::UpperRight],
            [Place::Left, Place::Center, Place::Right],
            [Place::LowerLeft, Place::Lower, Place::LowerRight],
            [Place::UpperLeft, Place::Left, Place::LowerLeft],
            [Place::Upper, Place::Center, Place::Lower],
            [Place::UpperRight, Place::Right, Place::LowerRight],
            [Place::UpperLeft, Place::Center, Place::LowerRight],
            [Place::UpperRight, Place::Center, Place::LowerLeft],
        ];

        let winner = triples.iter().filter_map(|&t| self.check_triple(t)).next();

        let all_filled = self.grid.iter().all(|p| p.is_some());

        if winner.is_some() || all_filled {
            Some(GameResult::from(winner))
        } else {
            None
        }
    }

    pub fn make_move(&mut self, player: Player, place: Place) -> Result<(), MoveError> {
        match self.status {
            Status::Finished(_) => Err(MoveError::InvalidStatus(self.status)),
            Status::Running(status_player) => {
                if let Some(place_player) = self[place] {
                    return Err(MoveError::PlaceAlreadyUsed(place, place_player));
                } else if status_player != player {
                    return Err(MoveError::WrongPlayer(status_player));
                }

                self.set_place(place, Some(player));

                self.status = if let Some(result) = self.check_win() {
                    Status::Finished(result)
                } else {
                    Status::Running(player.other())
                };

                Ok(())
            }
        }
    }
}

impl Default for TicTacToe {
    fn default() -> TicTacToe {
        TicTacToe::new()
    }
}

impl Index<Place> for TicTacToe {
    type Output = Option<Player>;

    fn index(&self, place: Place) -> &Option<Player> {
        &self.grid[TicTacToe::get_grid_index(place)]
    }
}
