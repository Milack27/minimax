use crate::*;

#[test]
fn test_2048() {
    let mut game = Game2048::new();

    assert_eq!(
        game.make_move(Move::Human(Direction::Up)),
        Err(MoveError::WrongPlayer(Player::Robot))
    );

    assert_eq!(
        game.make_move(Move::Robot(Place::from_xy(1, 1).unwrap(), 1)),
        Err(MoveError::ValueNotAllowed(1))
    );

    assert_eq!(
        game.make_move(Move::Robot(Place::from_xy(1, 1).unwrap(), 2)),
        Ok(())
    );

    assert_eq!(game.make_move(Move::Human(Direction::Down)), Ok(()));

    assert_eq!(
        game.make_move(Move::Robot(Place::from_xy(1, 0).unwrap(), 2)),
        Err(MoveError::PlaceAlreadyFilled(Place::from_xy(1, 0).unwrap()))
    );

    assert_eq!(
        game.make_move(Move::Robot(Place::from_xy(2, 3).unwrap(), 2)),
        Ok(())
    );

    assert_eq!(game.make_move(Move::Human(Direction::Down)), Ok(()));

    assert_eq!(
        game.make_move(Move::Robot(Place::from_xy(2, 3).unwrap(), 4)),
        Ok(())
    );

    assert_eq!(game.make_move(Move::Human(Direction::Left)), Ok(()));

    assert_eq!(
        game.make_move(Move::Robot(Place::from_xy(1, 3).unwrap(), 2)),
        Ok(())
    );

    assert_eq!(game.make_move(Move::Human(Direction::Down)), Ok(()));

    assert_eq!(game.get_status(), Status::Running(Player::Robot));

    for (i, value) in game.get_values().iter().cloned().enumerate() {
        match Place(i).get_xy() {
            (0, 0) => assert_eq!(value, 8),
            (1, 0) => assert_eq!(value, 2),
            _ => assert_eq!(value, 0),
        }
    }
}

#[test]
fn test_finish() {
    let mut game = Game2048 {
        status: Status::Running(Player::Robot),
        grid: [0, 4, 2, 4, 4, 2, 4, 2, 2, 4, 2, 4, 4, 2, 4, 2],
    };

    assert_eq!(
        game.make_move(Move::Robot(Place::from_xy(0, 0).unwrap(), 2)),
        Ok(())
    );

    assert_eq!(game.get_status(), Status::Finished);
}

#[test]
fn test_interleaved() {
    let mut game = Game2048::new();

    assert_eq!(
        game.make_move(Move::Robot(Place::from_xy(1, 0).unwrap(), 4)),
        Ok(())
    );
    assert_eq!(game.make_move(Move::Human(Direction::Left)), Ok(()));

    assert_eq!(
        game.make_move(Move::Robot(Place::from_xy(1, 1).unwrap(), 2)),
        Ok(())
    );
    assert_eq!(game.make_move(Move::Human(Direction::Left)), Ok(()));

    assert_eq!(
        game.make_move(Move::Robot(Place::from_xy(0, 3).unwrap(), 4)),
        Ok(())
    );
    assert_eq!(game.make_move(Move::Human(Direction::Down)), Ok(()));

    for (i, value) in game.get_values().iter().cloned().enumerate() {
        match Place(i).get_xy() {
            (0, 0) => assert_eq!(value, 4),
            (0, 1) => assert_eq!(value, 2),
            (0, 2) => assert_eq!(value, 4),
            _ => assert_eq!(value, 0),
        }
    }
}

#[test]
fn test_false_finish() {
    let mut game = Game2048 {
        status: Status::Running(Player::Robot),
        grid: [4, 8, 2, 2, 64, 128, 4, 4, 8, 8, 16, 2, 4, 4, 2, 0],
    };

    assert_eq!(
        game.make_move(Move::Robot(Place::from_xy(3, 3).unwrap(), 4)),
        Ok(())
    );

    assert_eq!(game.get_status(), Status::Running(Player::Human));
}
