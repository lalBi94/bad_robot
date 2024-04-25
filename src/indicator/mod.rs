#[derive(Debug, Copy, Clone)]
pub enum Indicator {
    WALL,
    NOTHING,
    PLAYER(char),
}

impl Indicator {
    pub fn is_wall(&self) -> bool {
        match self {
            Self::WALL => true,
            _ => false,
        }
    }

    pub fn char_to_indicator(flag: char) -> Self {
        match flag {
            '0' => Indicator::WALL,
            _ => Indicator::NOTHING,
        }
    }

    pub fn representation_of(&self) -> char {
        match self {
            Indicator::WALL => '█',
            Indicator::NOTHING => ' ',
            Indicator::PLAYER(v) => *v,
        }
    }
}
