use crate::*;

#[test]
fn test_core() {
    let mut game = TicTacToe::new();

    assert_eq!(game.make_move(Player::One, Place::Center), Ok(()));
    assert_eq!(game.make_move(Player::Two, Place::UpperLeft), Ok(()));
    assert_eq!(game.make_move(Player::One, Place::LowerLeft), Ok(()));

    assert_eq!(
        game.make_move(Player::One, Place::UpperRight),
        Err(MoveError::WrongPlayer(Player::Two))
    );

    assert_eq!(
        game.make_move(Player::Two, Place::UpperLeft),
        Err(MoveError::PlaceAlreadyUsed(Place::UpperLeft, Player::Two))
    );

    assert_eq!(game.make_move(Player::Two, Place::Upper), Ok(()));
    assert_eq!(game.make_move(Player::One, Place::UpperRight), Ok(()));

    assert_eq!(
        game.get_status(),
        Status::Finished(GameResult {
            winner: Some(Player::One)
        })
    );

    assert_eq!(
        game.make_move(Player::Two, Place::LowerRight),
        Err(MoveError::InvalidStatus(Status::Finished(GameResult {
            winner: Some(Player::One)
        })))
    );
}
