use crate::indicator;
use crate::neighbors;
use rand::Rng;
pub mod smart_robot_move;

#[derive(Debug, Clone)]
pub struct SmartRobot {
    pub current_case: [usize; 2],
    pub direction: Option<smart_robot_move::SmartRobotMove>,
}

impl SmartRobot {
    pub fn new(
        current_case: [usize; 2],
        direction: Option<smart_robot_move::SmartRobotMove>,
    ) -> Self {
        Self {
            current_case,
            direction,
        }
    }

    pub fn set_direction(&mut self, direction: smart_robot_move::SmartRobotMove) -> () {
        self.direction = Some(direction);
    }

    fn go_somewhere(&mut self, direction: smart_robot_move::SmartRobotMove) -> () {
        self.current_case = match direction {
            smart_robot_move::SmartRobotMove::LEFT => {
                [self.current_case[0], self.current_case[1] - 1]
            }
            smart_robot_move::SmartRobotMove::RIGHT => {
                [self.current_case[0], self.current_case[1] + 1]
            }
            smart_robot_move::SmartRobotMove::TOP => {
                [self.current_case[0] - 1, self.current_case[1]]
            }
            smart_robot_move::SmartRobotMove::DOWN => {
                [self.current_case[0] + 1, self.current_case[1]]
            }
        };
    }

    pub fn choose_direction(
        &mut self,
        neighbors: neighbors::Neighbors,
        predicted: bool,
    ) -> smart_robot_move::SmartRobotMove {
        let mut rng: rand::prelude::ThreadRng = rand::thread_rng();
        let available: Vec<smart_robot_move::SmartRobotMove> = neighbors.extract_possible_move();
        let random: usize = rng.gen_range(0..available.len());

        if !predicted {
            self.go_somewhere(available[random]);
        }

        available[random]
    }

    pub fn get_neighbors(&self, new_tray: &Vec<Vec<indicator::Indicator>>) -> neighbors::Neighbors {
        let mut stock_neighbors = [None; 4];

        if self.current_case[0] > 0 {
            if let Some(row) = new_tray.get(self.current_case[0] - 1) {
                if let Some(indicator) = row.get(self.current_case[1]) {
                    stock_neighbors[0] =
                        Some(([self.current_case[0] - 1, self.current_case[1]], *indicator));
                }
            }
        }

        if self.current_case[1] > 0 {
            if let Some(row) = new_tray.get(self.current_case[0]) {
                if let Some(indicator) = row.get(self.current_case[1] - 1) {
                    stock_neighbors[1] =
                        Some(([self.current_case[0], self.current_case[1] - 1], *indicator));
                }
            }
        }

        if self.current_case[1] + 1 < new_tray[0].len() {
            if let Some(row) = new_tray.get(self.current_case[0]) {
                if let Some(indicator) = row.get(self.current_case[1] + 1) {
                    stock_neighbors[2] =
                        Some(([self.current_case[0], self.current_case[1] + 1], *indicator));
                }
            }
        }

        if self.current_case[0] + 1 < new_tray.len() {
            if let Some(row) = new_tray.get(self.current_case[0] + 1) {
                if let Some(indicator) = row.get(self.current_case[1]) {
                    stock_neighbors[3] =
                        Some(([self.current_case[0] + 1, self.current_case[1]], *indicator));
                }
            }
        }

        neighbors::Neighbors::new(stock_neighbors)
    }
}
