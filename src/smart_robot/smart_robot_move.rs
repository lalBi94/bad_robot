#[derive(Debug, Clone, Copy)]
pub enum SmartRobotMove {
    LEFT,
    RIGHT,
    TOP,
    DOWN,
}

impl SmartRobotMove {
    pub fn get_arrow_from(&self) -> char {
        match self {
            Self::LEFT => '←',
            Self::RIGHT => '→',
            Self::TOP => '↑',
            Self::DOWN => '↓',
        }
    }
}
