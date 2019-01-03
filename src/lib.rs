use std::cmp::Ordering;
use std::hash::Hash;

use crate::GameResult::*;
use crate::Outcome::*;
use crate::Status::*;

#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq)]
pub enum Player {
    One,
    Two,
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

pub trait GameState: Clone {
    type Move: Eq + Hash;
    type MoveError;

    fn get_status(&self) -> Status;
    fn possible_moves(&self) -> Vec<Self::Move>;
    fn make_move(&mut self, mov: &Self::Move) -> Result<(), Self::MoveError>;

    fn get_score(&self) -> isize {
        0
    }

    fn minimax(&self, depth: usize) -> Result<Vec<Self::Move>, MinimaxError<Self>> {
        Ok(minimax(self, depth)?.1)
    }
}

#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq)]
pub enum MinimaxError<S: GameState> {
    GameAlreadyFinished,
    MoveError(S::MoveError),
    NoPossibleMoves,
}

#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq)]
enum Outcome {
    Definite(GameResult, usize),
    Indefinite(isize),
}

impl Player {
    pub fn other(self) -> Player {
        match self {
            Player::One => Player::Two,
            Player::Two => Player::One,
        }
    }
}

impl GameResult {
    pub fn winner(self) -> Option<Player> {
        match self {
            GameResult::Draw => None,
            GameResult::Win(player) => Some(player),
        }
    }
}

fn compare_outcome(player: Player, lhs: &Outcome, rhs: &Outcome) -> Ordering {
    if lhs == rhs {
        return Ordering::Equal;
    }

    let normalize = |score: &isize| match player {
        Player::One => *score,
        Player::Two => -score,
    };

    let greater_if = |b| if b { Ordering::Greater } else { Ordering::Less };

    match (lhs, rhs) {
        (Definite(Win(lhs_winner), lhs_moves), Definite(Win(rhs_winner), rhs_moves))
            if (*lhs_winner, *rhs_winner) == (player, player) =>
        {
            lhs_moves.cmp(rhs_moves).reverse()
        }
        (Definite(Win(lhs_winner), lhs_moves), Definite(Win(rhs_winner), rhs_moves))
            if (*lhs_winner, *rhs_winner) == (player.other(), player.other()) =>
        {
            lhs_moves.cmp(rhs_moves)
        }
        (Definite(Win(winner), _), _) => greater_if(*winner == player),
        (Definite(Draw, lhs_moves), Definite(Draw, rhs_moves)) => lhs_moves.cmp(rhs_moves),
        (Definite(Draw, _), Indefinite(score)) => greater_if(normalize(score).is_negative()),
        (Indefinite(lhs_score), Indefinite(rhs_score)) => {
            normalize(lhs_score).cmp(&normalize(rhs_score))
        }
        _ => compare_outcome(player, rhs, lhs).reverse(),
    }
}

fn minimax<S: GameState>(
    state: &S,
    depth: usize,
) -> Result<(Outcome, Vec<S::Move>), MinimaxError<S>> {
    let player = match state.get_status() {
        Running(player) => player,
        Finished(_) => return Err(MinimaxError::GameAlreadyFinished),
    };

    let outcomes = state
        .possible_moves()
        .into_iter()
        .map(|mov| {
            let mut child_state = state.clone();

            child_state
                .make_move(&mov)
                .map_err(MinimaxError::MoveError)?;

            let outcome = match child_state.get_status() {
                Status::Finished(result) => Definite(result, 0),
                Status::Running(_) if depth == 0 => Indefinite(child_state.get_score()),
                _ => {
                    let child_outcome = minimax(&child_state, depth - 1)?.0;

                    match child_outcome {
                        Definite(result, moves) => Definite(result, moves + 1),
                        _ => child_outcome,
                    }
                }
            };

            Ok((mov, outcome))
        })
        .collect::<Result<Vec<_>, MinimaxError<_>>>()?;

    let best_outcome = outcomes
        .iter()
        .max_by(|(_, l), (_, r)| compare_outcome(player, l, r))
        .ok_or(MinimaxError::NoPossibleMoves)?
        .1;

    let moves = outcomes
        .into_iter()
        .filter(|(_mov, outcome)| *outcome == best_outcome)
        .map(|(mov, _outcome)| mov)
        .collect();

    Ok((best_outcome, moves))
}
