#![feature(try_from)]

mod display;
mod minimax;

#[cfg(test)]
mod test;

use std::iter::once;
use std::ops::Add;
use std::ops::Index;

const GRID_WIDTH: usize = 4;
const GRID_HEIGHT: usize = 4;
const GRID_SIZE: usize = GRID_WIDTH * GRID_HEIGHT;

#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq)]
pub struct Place(usize);

#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq)]
pub struct InvalidPlace;

#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq)]
pub enum Player {
    Human,
    Robot,
}

#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq)]
pub enum Direction {
    Up,
    Down,
    Left,
    Right,
}

#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq)]
pub enum Move {
    Human(Direction),
    Robot(Place, usize),
}

#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq)]
pub enum Status {
    Running(Player),
    Finished,
}

#[derive(Clone, Debug, Hash, PartialEq, Eq)]
pub struct Game2048 {
    status: Status,
    grid: [usize; GRID_SIZE],
}

#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq)]
pub enum MoveError {
    InvalidStatus(Status),
    WrongPlayer(Player),
    PlaceAlreadyFilled(Place),
    ValueNotAllowed(usize),
    DirectionBlocked(Direction),
}

impl Place {
    pub fn from_xy(x: usize, y: usize) -> Result<Place, InvalidPlace> {
        if x >= GRID_WIDTH || y >= GRID_HEIGHT {
            Err(InvalidPlace)
        } else {
            Ok(Place(y * GRID_WIDTH + x))
        }
    }

    pub fn get_xy(self) -> (usize, usize) {
        (self.0 % GRID_WIDTH, self.0 / GRID_WIDTH)
    }
}

impl Player {
    pub fn other(self) -> Player {
        match self {
            Player::Human => Player::Robot,
            Player::Robot => Player::Human,
        }
    }
}

impl Direction {
    pub fn values() -> impl Iterator<Item = Direction> {
        [
            Direction::Up,
            Direction::Down,
            Direction::Left,
            Direction::Right,
        ]
        .iter()
        .cloned()
    }

    pub fn opposite(self) -> Direction {
        match self {
            Direction::Up => Direction::Down,
            Direction::Down => Direction::Up,
            Direction::Left => Direction::Right,
            Direction::Right => Direction::Left,
        }
    }
}

impl Move {
    pub fn get_player(self) -> Player {
        match self {
            Move::Human(_) => Player::Human,
            Move::Robot(_, _) => Player::Robot,
        }
    }
}

impl Game2048 {
    pub fn new() -> Game2048 {
        Game2048 {
            status: Status::Running(Player::Robot),
            grid: [0; GRID_SIZE],
        }
    }

    pub fn get_status(&self) -> Status {
        self.status
    }

    pub fn get_value(&self, place: Place) -> usize {
        self.grid[place.0]
    }

    pub fn get_values(&self) -> &[usize] {
        &self.grid
    }

    fn set_value(&mut self, place: Place, value: usize) {
        self.grid[place.0] = value;
    }

    fn check_direction_mobility(&self, direction: Direction) -> bool {
        self.grid
            .iter()
            .enumerate()
            .filter(|p| *p.1 > 0)
            .map(|p| Place(p.0))
            .any(|p| match p + direction {
                Err(_) => false,
                Ok(adjacent) => self[adjacent] == 0 || self[adjacent] == self[p],
            })
    }

    pub fn possible_moves(&self) -> Vec<Move> {
        match self.status {
            Status::Finished => Vec::new(),
            Status::Running(Player::Human) => Direction::values()
                .filter(|&d| self.check_direction_mobility(d))
                .map(Move::Human)
                .collect(),
            Status::Running(Player::Robot) => self
                .grid
                .iter()
                .enumerate()
                .filter(|p| *p.1 == 0)
                .flat_map(|p| {
                    once(Move::Robot(Place(p.0), 2)).chain(once(Move::Robot(Place(p.0), 4)))
                })
                .collect(),
        }
    }

    pub fn make_move(&mut self, mov: Move) -> Result<(), MoveError> {
        match self.status {
            Status::Finished => Err(MoveError::InvalidStatus(self.status)),
            Status::Running(player) => {
                if player != mov.get_player() {
                    return Err(MoveError::WrongPlayer(player));
                }

                match mov {
                    Move::Human(direction) => self.make_human_move(direction),
                    Move::Robot(place, value) => self.make_robot_move(place, value),
                }
            }
        }
    }

    fn get_lines(direction: Direction) -> impl Iterator<Item = Place> {
        let range = match direction {
            Direction::Up | Direction::Down => 0..GRID_WIDTH,
            Direction::Left | Direction::Right => 0..GRID_HEIGHT,
        };

        let mapper: fn(usize) -> Result<Place, InvalidPlace> = match direction {
            Direction::Up => |x| Place::from_xy(x, GRID_HEIGHT - 1),
            Direction::Down => |x| Place::from_xy(x, 0),
            Direction::Left => |y| Place::from_xy(0, y),
            Direction::Right => |y| Place::from_xy(GRID_WIDTH - 1, y),
        };

        range.map(move |n| mapper(n).expect("Invalid line head."))
    }

    fn make_human_move(&mut self, direction: Direction) -> Result<(), MoveError> {
        let opposite_direction = direction.opposite();

        let mut changed = false;

        for head in Game2048::get_lines(direction) {
            let mut write_cursor = head;
            let mut read_cursor_result = head + opposite_direction;

            while let Ok(read_cursor) = read_cursor_result {
                let read_value = self[read_cursor];
                let write_value = self[write_cursor];
                let next_write_cursor =
                    (write_cursor + opposite_direction).expect("Invalid write cursor.");

                read_cursor_result = read_cursor + opposite_direction;

                if read_value == 0 {
                    continue;
                } else if write_value == 0 {
                    self.set_value(write_cursor, read_value);
                } else if read_value == write_value {
                    self.set_value(write_cursor, 2 * write_value);
                    write_cursor = next_write_cursor;
                } else if next_write_cursor == read_cursor {
                    write_cursor = next_write_cursor;
                    continue;
                } else {
                    write_cursor = next_write_cursor;
                    self.set_value(write_cursor, read_value);
                }

                self.set_value(read_cursor, 0);
                changed = true;
            }
        }

        if changed {
            self.status = Status::Running(Player::Robot);
            Ok(())
        } else {
            Err(MoveError::DirectionBlocked(direction))
        }
    }

    fn make_robot_move(&mut self, place: Place, value: usize) -> Result<(), MoveError> {
        if self[place] > 0 {
            return Err(MoveError::PlaceAlreadyFilled(place));
        }

        if ![2, 4].contains(&value) {
            return Err(MoveError::ValueNotAllowed(value));
        }

        self.set_value(place, value);
        self.status = Status::Running(Player::Human);

        if self.possible_moves().is_empty() {
            self.status = Status::Finished
        }

        Ok(())
    }
}

impl Default for Game2048 {
    fn default() -> Game2048 {
        Game2048::new()
    }
}

impl Index<Place> for Game2048 {
    type Output = usize;

    fn index(&self, place: Place) -> &usize {
        &self.grid[place.0]
    }
}

impl Add<Direction> for Place {
    type Output = Result<Place, InvalidPlace>;

    fn add(self, direction: Direction) -> Result<Place, InvalidPlace> {
        let (mut x, mut y) = self.get_xy();

        match direction {
            Direction::Up => y = y.wrapping_add(1),
            Direction::Down => y = y.wrapping_sub(1),
            Direction::Left => x = x.wrapping_sub(1),
            Direction::Right => x = x.wrapping_add(1),
        }

        Place::from_xy(x, y)
    }
}
