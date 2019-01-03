use ::minimax::{GameState, MinimaxError};

use std::io;

use tic_tac_toe::*;

use tic_tac_toe::GameResult::*;
use tic_tac_toe::MoveError::*;
use tic_tac_toe::Place::*;
use tic_tac_toe::Status::*;

enum TicTacToeError {
    MoveError(MoveError),
    InvalidInput(String),
}

impl From<MoveError> for TicTacToeError {
    fn from(error: MoveError) -> TicTacToeError {
        TicTacToeError::MoveError(error)
    }
}

fn print_instructions() {
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
}

fn print_minimax(game: &TicTacToe) -> Result<(), MinimaxError<TicTacToe>> {
    const MINIMAX_DEPTH: usize = 5;

    print!("Minimax: ");

    let places = match game.minimax(MINIMAX_DEPTH) {
        Ok(places) => places,
        Err(error) => {
            println!("{:?}\n", error);
            return Err(error);
        }
    };

    for (i, place) in places.iter().enumerate() {
        print!(
            "{}",
            match place {
                UpperLeft => "Q",
                Upper => "W",
                UpperRight => "E",
                Left => "A",
                Center => "S",
                Right => "D",
                LowerLeft => "Z",
                Lower => "X",
                LowerRight => "C",
            }
        );

        if i < places.len() {
            print!(", ");
        }
    }

    println!("\n");
    Ok(())
}

fn parse_input(input: String) -> Result<Place, TicTacToeError> {
    match input.as_str() {
        "Q" => Ok(UpperLeft),
        "W" => Ok(Upper),
        "E" => Ok(UpperRight),
        "A" => Ok(Left),
        "S" => Ok(Center),
        "D" => Ok(Right),
        "Z" => Ok(LowerLeft),
        "X" => Ok(Lower),
        "C" => Ok(LowerRight),
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
        TicTacToeError::MoveError(InvalidStatus(_)) => {
            panic!("Cannot make any move now because the status of the game doesn't allow it.");
        }
        TicTacToeError::MoveError(WrongPlayer(_)) => {
            panic!("Cannot make that move because it's not the player's turn.");
        }
        TicTacToeError::MoveError(PlaceAlreadyUsed(_, _)) => {
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
    print_instructions();

    let mut game = TicTacToe::new();

    loop {
        match game.get_status() {
            Running(player) => {
                print_minimax(&game).ok();

                let result = handle_turn(&mut game, player);

                if let Err(error) = result {
                    handle_error(error);
                }

                println!("{}", game);
            }
            Finished(result) => {
                match result {
                    Draw => println!("Draw."),
                    Win(Player::One) => println!("X wins."),
                    Win(Player::Two) => println!("O wins."),
                }

                break;
            }
        }
    }
}
