use std::io;

use tic_tac_toe::*;

enum TicTacToeError {
    MoveError(MoveError),
    InvalidInput(String),
}

impl From<MoveError> for TicTacToeError {
    fn from(error: MoveError) -> TicTacToeError {
        TicTacToeError::MoveError(error)
    }
}

fn parse_input(input: String) -> Result<Place, TicTacToeError> {
    match input.as_str() {
        "Q" => Ok(Place::UpperLeft),
        "W" => Ok(Place::Upper),
        "E" => Ok(Place::UpperRight),
        "A" => Ok(Place::Left),
        "S" => Ok(Place::Center),
        "D" => Ok(Place::Right),
        "Z" => Ok(Place::LowerLeft),
        "X" => Ok(Place::Lower),
        "C" => Ok(Place::LowerRight),
        _ => Err(TicTacToeError::InvalidInput(input)),
    }
}

fn handle_turn(game: &mut TicTacToe, player: Player) -> Result<(), TicTacToeError> {
    let input = {
        let mut buffer = String::new();

        io::stdin()
            .read_line(&mut buffer)
            .expect("It was not possible to read user input.");

        println!();

        buffer.trim().to_uppercase()
    };

    let place = parse_input(input)?;

    game.make_move(player, place)?;
    Ok(())
}

fn handle_error(error: TicTacToeError) {
    match error {
        TicTacToeError::MoveError(MoveError::InvalidStatus(_)) => {
            panic!("Cannot make any move now because the status of the game doesn't allow it.");
        }
        TicTacToeError::MoveError(MoveError::WrongPlayer(_)) => {
            panic!("Cannot make that move because it's not the player's turn.");
        }
        TicTacToeError::MoveError(MoveError::PlaceAlreadyUsed(_, _)) => {
            println!("Cannot make that move because that place is already used.");
        }
        TicTacToeError::InvalidInput(input) => {
            println!("Invalid input: {}", input);
            println!("Please, enter one of the following: Q, W, E, A, S, D, Z, X, or C.");
        }
    }

    println!();
}

fn main() {
    println!("-----------------------------");
    println!("         TIC TAC TOE         ");
    println!("-----------------------------");
    println!();
    println!("Press the following keys and ENTER to fill the blank places:");
    println!();
    println!(" Q | W | E ");
    println!("---+---+---");
    println!(" A | S | D ");
    println!("---+---+---");
    println!(" Z | X | C ");
    println!();

    let mut game = TicTacToe::new();

    loop {
        match game.get_status() {
            Status::Running(player) => {
                let result = handle_turn(&mut game, player);

                if let Err(error) = result {
                    handle_error(error);
                }

                println!("{}", game);
            }
            Status::Finished(result) => {
                match result.get_winner() {
                    None => println!("Draw."),
                    Some(Player::One) => println!("X wins."),
                    Some(Player::Two) => println!("O wins."),
                }

                break;
            }
        }
    }
}
