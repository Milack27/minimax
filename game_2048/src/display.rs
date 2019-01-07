use crate::*;

use std::fmt;

fn print_markers(
    f: &mut fmt::Formatter,
    digits: usize,
    width: usize,
    border: char,
    marker: char,
    space: char,
) -> fmt::Result {
    write!(f, "{}", border)?;

    for x in 0..width {
        for _ in 0..(digits + 2) {
            write!(f, "{}", space)?;
        }

        if x < width - 1 {
            write!(f, "{}", marker)?;
        }
    }

    writeln!(f, "{}", border)
}

fn print_horizontal_border(f: &mut fmt::Formatter, digits: usize, width: usize) -> fmt::Result {
    print_markers(f, digits, width, '+', '+', '-')
}

fn print_line(
    f: &mut fmt::Formatter,
    digits: usize,
    values: impl Iterator<Item = usize>,
    border: char,
) -> fmt::Result {
    write!(f, "{}", border)?;

    for (i, value) in values.enumerate() {
        if value == 0 {
            write!(f, "{:^1$}", "", digits + 2)?;
        } else {
            write!(f, "{:^1$}", value, digits + 2)?;
        }

        if i < GRID_WIDTH - 1 {
            write!(f, " ")?;
        }
    }

    writeln!(f, "{}", border)
}

impl fmt::Display for Game2048 {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let max_value = self
            .get_values()
            .iter()
            .cloned()
            .max()
            .expect("Empty grid.");

        let max_digits = max_value.to_string().len();

        print_horizontal_border(f, max_digits, GRID_WIDTH)?;

        for y in 0..GRID_HEIGHT {
            let values = (0..GRID_WIDTH)
                .map(|x| self[Place::from_xy(x, GRID_HEIGHT - y - 1).expect("Invalid place.")]);

            print_line(f, max_digits, values, '|')?;

            if y < GRID_HEIGHT - 1 {
                print_markers(f, max_digits, GRID_WIDTH, '+', '+', ' ')?;
            }
        }

        print_horizontal_border(f, max_digits, GRID_WIDTH)?;
        Ok(())
    }
}
