use crate::GameResult::*;
use crate::MoveError::*;
use crate::Status::*;
use crate::*;

#[test]
fn test_tic_tac_toe() {
    let mut game = TicTacToe::new();

    assert_eq!(game.make_move(Player::One, Place::Center), Ok(()));
    assert_eq!(game.make_move(Player::Two, Place::UpperLeft), Ok(()));
    assert_eq!(game.make_move(Player::One, Place::LowerLeft), Ok(()));

    assert_eq!(
        game.make_move(Player::One, Place::UpperRight),
        Err(WrongPlayer(Player::Two))
    );

    assert_eq!(
        game.make_move(Player::Two, Place::UpperLeft),
        Err(PlaceAlreadyUsed(Place::UpperLeft, Player::Two))
    );

    assert_eq!(game.make_move(Player::Two, Place::Upper), Ok(()));
    assert_eq!(game.make_move(Player::One, Place::UpperRight), Ok(()));

    assert_eq!(game.get_status(), Finished(Win(Player::One)));

    assert_eq!(
        game.make_move(Player::Two, Place::LowerRight),
        Err(InvalidStatus(Finished(Win(Player::One))))
    );
}
