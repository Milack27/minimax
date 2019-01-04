use crate::GameResult::*;
use crate::MoveError::*;
use crate::Status::*;
use crate::*;

#[test]
fn test_tic_tac_toe() {
    let mut game = TicTacToe::new();

    assert_eq!(game.make_move(Player::X, Place::Center), Ok(()));
    assert_eq!(game.make_move(Player::O, Place::UpperLeft), Ok(()));
    assert_eq!(game.make_move(Player::X, Place::LowerLeft), Ok(()));

    assert_eq!(
        game.make_move(Player::X, Place::UpperRight),
        Err(WrongPlayer(Player::O))
    );

    assert_eq!(
        game.make_move(Player::O, Place::UpperLeft),
        Err(PlaceAlreadyUsed(Place::UpperLeft, Player::O))
    );

    assert_eq!(game.make_move(Player::O, Place::Upper), Ok(()));
    assert_eq!(game.make_move(Player::X, Place::UpperRight), Ok(()));

    assert_eq!(game.get_status(), Finished(Win(Player::X)));

    assert_eq!(
        game.make_move(Player::O, Place::LowerRight),
        Err(InvalidStatus(Finished(Win(Player::X))))
    );
}
