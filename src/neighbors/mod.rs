use crate::indicator;
use crate::smart_robot::smart_robot_move;

#[derive(Debug, Copy, Clone)]
pub struct Neighbors {
    top: Option<([usize; 2], indicator::Indicator)>,
    left: Option<([usize; 2], indicator::Indicator)>,
    right: Option<([usize; 2], indicator::Indicator)>,
    bottom: Option<([usize; 2], indicator::Indicator)>,
}

impl Neighbors {
    pub fn new(neighbors: [Option<([usize; 2], indicator::Indicator)>; 4]) -> Self {
        Self {
            top: if let Some(td) = neighbors[0] {
                match td.1 {
                    indicator::Indicator::WALL => None,
                    _ => Some(td),
                }
            } else {
                None
            },
            left: if let Some(ld) = neighbors[1] {
                match ld.1 {
                    indicator::Indicator::WALL => None,
                    _ => Some(ld),
                }
            } else {
                None
            },
            right: if let Some(rd) = neighbors[2] {
                match rd.1 {
                    indicator::Indicator::WALL => None,
                    _ => Some(rd),
                }
            } else {
                None
            },
            bottom: if let Some(bd) = neighbors[3] {
                match bd.1 {
                    indicator::Indicator::WALL => None,
                    _ => Some(bd),
                }
            } else {
                None
            },
        }
    }

    pub fn extract_possible_move(&self) -> Vec<smart_robot_move::SmartRobotMove> {
        let mut extracted: Vec<smart_robot_move::SmartRobotMove> = Vec::new();

        if let Some(_) = self.top {
            extracted.push(smart_robot_move::SmartRobotMove::TOP);
        };
        if let Some(_) = self.left {
            extracted.push(smart_robot_move::SmartRobotMove::LEFT);
        };
        if let Some(_) = self.right {
            extracted.push(smart_robot_move::SmartRobotMove::RIGHT);
        };
        if let Some(_) = self.bottom {
            extracted.push(smart_robot_move::SmartRobotMove::DOWN);
        };

        extracted
    }
}
