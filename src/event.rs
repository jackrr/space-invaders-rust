#[derive(PartialEq, Eq, Debug)]
pub enum EventKind {
    EnemyMove,
    PlayerMove,
}

#[derive(PartialEq, Eq, Debug)]
pub enum Direction {
    Left,
    Right,
}

pub struct Event {
    pub kind: EventKind,
    pub direction: Option<Direction>,
}

impl Event {
    pub fn player_move(direction: Direction) -> Self {
        Self {
            kind: EventKind::PlayerMove,
            direction: Some(direction),
        }
    }

    pub fn enemy_move() -> Self {
        Self {
            kind: EventKind::EnemyMove,
            direction: None,
        }
    }
}
