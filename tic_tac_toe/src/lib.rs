use std::ops::Index;

mod display;

#[cfg(test)]
mod test;

#[derive(Clone, Copy, Debug, Hash, PartialEq)]
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

#[derive(Clone, Copy, Debug, Hash, PartialEq)]
pub enum Player {
    One,
    Two,
}

#[derive(Clone, Copy, Debug, Hash, PartialEq)]
pub enum GameResult {
    Draw,
    Win(Player),
}

#[derive(Clone, Copy, Debug, Hash, PartialEq)]
pub enum Status {
    Running(Player),
    Finished(GameResult),
}

#[derive(Clone, Debug, Hash, PartialEq)]
pub struct TicTacToe {
    status: Status,
    grid: [Option<Player>; 9],
}

#[derive(Clone, Copy, Debug, Hash, PartialEq)]
pub enum MoveError {
    InvalidStatus(Status),
    WrongPlayer(Player),
    PlaceAlreadyUsed(Place, Player),
    EmptyPlace(Place),
}

impl Player {
    fn other(self) -> Player {
        match self {
            Player::One => Player::Two,
            Player::Two => Player::One,
        }
    }
}

impl GameResult {
    pub fn from_winner(winner: Option<Player>) -> GameResult {
        match winner {
            None => GameResult::Draw,
            Some(player) => GameResult::Win(player),
        }
    }

    pub fn get_winner(self) -> Option<Player> {
        match self {
            GameResult::Draw => None,
            GameResult::Win(player) => Some(player),
        }
    }
}

impl TicTacToe {
    pub fn new() -> TicTacToe {
        TicTacToe {
            status: Status::Running(Player::One),
            grid: [None, None, None, None, None, None, None, None, None],
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
                .filter_map(|t| t.1.map(|_| TicTacToe::get_grid_place(t.0)))
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
            Some(GameResult::from_winner(winner))
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

    pub fn revert_move(&mut self, player: Player, place: Place) -> Result<(), MoveError> {
        match self[place] {
            None => Err(MoveError::EmptyPlace(place)),
            Some(place_player) => {
                if player != place_player {
                    return Err(MoveError::WrongPlayer(place_player));
                }

                self.set_place(place, None);

                self.status = if let Some(result) = self.check_win() {
                    Status::Finished(result)
                } else {
                    Status::Running(player)
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
