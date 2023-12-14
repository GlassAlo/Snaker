pub(crate) enum Direction {
    Up,
    Right,
    Down,
    Left,
}

impl Direction {
    pub(crate) fn rotation_angle(&self) -> f32 {
        match self {
            Direction::Up => 90.0,
            Direction::Right => 0.0,
            Direction::Down => 270.0,
            Direction::Left => 180.0,
        }
    }
}
