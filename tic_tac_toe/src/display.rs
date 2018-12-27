use crate::*;

use std::fmt;

fn get_char(player: Option<Player>) -> char {
    match player {
        None => ' ',
        Some(Player::One) => 'X',
        Some(Player::Two) => 'O',
    }
}

impl fmt::Display for Player {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", get_char(Some(*self)))
    }
}

impl fmt::Display for TicTacToe {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let upper_left = get_char(self[Place::UpperLeft]);
        let upper = get_char(self[Place::Upper]);
        let upper_right = get_char(self[Place::UpperRight]);
        let left = get_char(self[Place::Left]);
        let center = get_char(self[Place::Center]);
        let right = get_char(self[Place::Right]);
        let lower_left = get_char(self[Place::LowerLeft]);
        let lower = get_char(self[Place::Lower]);
        let lower_right = get_char(self[Place::LowerRight]);

        writeln!(f, " {} | {} | {} ", upper_left, upper, upper_right)?;
        writeln!(f, "---+---+---")?;
        writeln!(f, " {} | {} | {} ", left, center, right)?;
        writeln!(f, "---+---+---")?;
        writeln!(f, " {} | {} | {} ", lower_left, lower, lower_right)?;

        Ok(())
    }
}
