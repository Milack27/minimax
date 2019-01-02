use ::minimax::{ GameState, MinimaxError };

use std::io;

use game_2048::*;

use game_2048::Player::*;
use game_2048::Direction::*;
use game_2048::MoveError::*;

enum Game2048Error {
    MoveError(MoveError),
    InvalidInput(String),
}

impl From<MoveError> for Game2048Error {
    fn from(error: MoveError) -> Game2048Error {
        Game2048Error::MoveError(error)
    }
}

fn print_instructions() {
    println!("-----------------------------");
    println!("             2048            ");
    println!("-----------------------------");
    println!();
    println!("Enter the robot move in the following format: <place>, <value>");
    println!("Where <value> is 2 or 4, and <place> is one the following characters:");
    println!();
    println!("+---+---+---+---+");
    println!("| 1   2   3   4 |");
    println!("|   +   +   +   |");
    println!("| Q   W   E   R |");
    println!("|   +   +   +   |");
    println!("| A   S   D   F |");
    println!("|   +   +   +   |");
    println!("| Z   X   C   V |");
    println!("+---+---+---+---+");
    println!();
    println!("Enter the human move in the following format: <direction>");
    println!("Where <direction> is one of the following characters:");
    println!();
    println!("W: Up");
    println!("A: Left");
    println!("S: Down");
    println!("D: Right");
    println!();
}

fn print_minimax(game: &Game2048) -> Result<(), MinimaxError<Game2048>> {
    const MINIMAX_DEPTH: usize = 5;

    println!("Minimax:");

    let moves = match game.minimax(MINIMAX_DEPTH) {
        Ok(moves) => moves,
        Err(error) => {
            println!("{:?}\n", error);
            return Err(error);
        }
    };
    
    for mov in moves {
        match mov {
            Move::Human(Up) => println!("W"),
            Move::Human(Left) => println!("A"),
            Move::Human(Down) => println!("S"),
            Move::Human(Right) => println!("D"),

            Move::Robot(place, value) => match place.get_xy() {
                (0, 0) => println!("Z, {}", value),
                (1, 0) => println!("X, {}", value),
                (2, 0) => println!("C, {}", value),
                (3, 0) => println!("V, {}", value),

                (0, 1) => println!("A, {}", value),
                (1, 1) => println!("S, {}", value),
                (2, 1) => println!("D, {}", value),
                (3, 1) => println!("F, {}", value),

                (0, 2) => println!("Q, {}", value),
                (1, 2) => println!("W, {}", value),
                (2, 2) => println!("E, {}", value),
                (3, 2) => println!("R, {}", value),

                (0, 3) => println!("1, {}", value),
                (1, 3) => println!("2, {}", value),
                (2, 3) => println!("3, {}", value),
                (3, 3) => println!("4, {}", value),

                _ => println!("(unknown)"),
            }
        };
    }

    println!();
    Ok(())
}

fn handle_human_turn(game: &mut Game2048, input: String) -> Result<(), Game2048Error> {
    let direction = match input.as_str() {
        "W" => Ok(Up),
        "A" => Ok(Left),
        "S" => Ok(Down),
        "D" => Ok(Right),
        _ => Err(Game2048Error::InvalidInput(input)),
    }?;

    game.make_move(Move::Human(direction))?;
    Ok(())
}

fn handle_robot_turn(game: &mut Game2048, input: String) -> Result<(), Game2048Error> {
    let mut input_iterator = input.split(',');

    let place = input_iterator.next();
    let value = input_iterator.next();

    let place = match place.map(|s| s.trim()) {
        Some("1") => Place::from_xy(0, 3).unwrap(),
        Some("2") => Place::from_xy(1, 3).unwrap(),
        Some("3") => Place::from_xy(2, 3).unwrap(),
        Some("4") => Place::from_xy(3, 3).unwrap(),

        Some("Q") => Place::from_xy(0, 2).unwrap(),
        Some("W") => Place::from_xy(1, 2).unwrap(),
        Some("E") => Place::from_xy(2, 2).unwrap(),
        Some("R") => Place::from_xy(3, 2).unwrap(),

        Some("A") => Place::from_xy(0, 1).unwrap(),
        Some("S") => Place::from_xy(1, 1).unwrap(),
        Some("D") => Place::from_xy(2, 1).unwrap(),
        Some("F") => Place::from_xy(3, 1).unwrap(),

        Some("Z") => Place::from_xy(0, 0).unwrap(),
        Some("X") => Place::from_xy(1, 0).unwrap(),
        Some("C") => Place::from_xy(2, 0).unwrap(),
        Some("V") => Place::from_xy(3, 0).unwrap(),

        _ => return Err(Game2048Error::InvalidInput(input)),
    };

    let value = match value.map(|s| s.trim().parse::<usize>()) {
        Some(Ok(2)) => 2,
        Some(Ok(4)) => 4,
        _ => return Err(Game2048Error::InvalidInput(input)),
    };

    game.make_move(Move::Robot(place, value))?;
    Ok(())
}

fn handle_error(error: Game2048Error) {
    match error {
        Game2048Error::MoveError(InvalidStatus(_)) => {
            panic!("Cannot make any move now because the status of the game doesn't allow it.");
        }
        Game2048Error::MoveError(WrongPlayer(_)) => {
            panic!("Cannot make that move because it's not the player's turn.");
        }
        Game2048Error::MoveError(PlaceAlreadyFilled(_)) => {
            println!("Cannot make that move because that place is already used.");
        }
        Game2048Error::MoveError(ValueNotAllowed(_)) => {
            println!(
                "Cannot make that move because the given value is not allowed. Use only 2 or 4."
            );
        }
        Game2048Error::MoveError(DirectionBlocked(_)) => {
            println!("Cannot make that move because the given direction is blocked.");
        }
        Game2048Error::InvalidInput(input) => {
            println!("Invalid input: {}", input);
        }
    }

    println!();
}

fn main() {
    print_instructions();

    let mut game = Game2048::new();

    while let Status::Running(player) = game.get_status() {
        print_minimax(&game).ok();

        let input = {
            let mut buffer = String::new();

            io::stdin()
                .read_line(&mut buffer)
                .expect("It was not possible to read user input.");

            println!();

            buffer.trim().to_uppercase()
        };

        let result = match player {
            Human => handle_human_turn(&mut game, input),
            Robot => handle_robot_turn(&mut game, input),
        };

        if let Err(error) = result {
            handle_error(error);
        }

        println!("{}", game);
    }
}
