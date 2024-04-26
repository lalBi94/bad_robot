use crate::indicator;
use crate::neighbors;
use crate::smart_robot;
use std::time::Instant;
use std::{process, thread::sleep, time::Duration};
pub mod maze_tools;

#[derive(Debug, Clone)]
pub struct Maze {
    tray: Vec<Vec<indicator::Indicator>>,
    end: [usize; 2],
    smart_robot: smart_robot::SmartRobot,
}

impl Maze {
    pub fn new(tray: Vec<String>) -> Self {
        let pack = Maze::prelude(tray);

        Self {
            tray: pack.0,
            end: pack.1,
            smart_robot: pack.2,
        }
    }

    // Stop here if right side dont contain any enter
    pub fn found_start(tray: &Vec<Vec<indicator::Indicator>>) -> [usize; 2] {
        for i in 0..tray.len() {
            if !tray[i][0].is_wall() {
                return [i, 0];
            }
        }

        process::exit(-1)
    }

    // Stop here if right side dont contain any exit
    pub fn found_end(tray: &Vec<Vec<indicator::Indicator>>) -> [usize; 2] {
        for i in 0..tray.len() {
            if !tray[i][tray[i].len() - 1].is_wall() {
                return [i, tray[i].len() - 1];
            }
        }

        process::exit(-1)
    }

    fn prelude(
        tray: Vec<String>,
    ) -> (
        Vec<Vec<indicator::Indicator>>,
        [usize; 2],
        smart_robot::SmartRobot,
    ) {
        let mut new_tray: Vec<Vec<indicator::Indicator>> = Vec::new();

        for s in tray.iter() {
            let mut stock: Vec<indicator::Indicator> = Vec::new();

            for c in s.chars() {
                stock.push(indicator::Indicator::char_to_indicator(c))
            }

            new_tray.push(stock);
        }

        let f_start: [usize; 2] = Self::found_start(&new_tray.to_vec());
        let mut bot = smart_robot::SmartRobot::new([f_start[0], f_start[1]], None);
        let neighbors: neighbors::Neighbors = bot.get_neighbors(&new_tray);
        let direction = bot.choose_direction(neighbors, true);
        bot.set_direction(direction);

        (new_tray.to_vec(), Self::found_end(&new_tray.to_vec()), bot)
    }

    pub fn display(&self) -> () {
        for e in self.tray.iter() {
            for i in e.iter() {
                print!("{}", i.representation_of())
            }

            println!();
        }

        println!();
    }

    fn feedback_average(average: &usize) -> f64 {
        if *average == 0 {
            return 0.0;
        };

        if *average as f64 > 1700.0 {
            0.1 // +0.1 gift
        } else if *average as f64 > 1600.0 {
            1.0
        } else if *average as f64 > 1500.0 {
            2.0
        } else if *average as f64 > 1400.0 {
            3.0
        } else if *average as f64 > 1300.0 {
            4.0
        } else if *average as f64 > 1200.0 {
            5.0
        } else if *average as f64 > 1000.0 {
            6.0 + (*average as f64 - 1000.0) / 500.0 * 2.0
        } else if *average as f64 > 950.0 {
            8.0 + (*average as f64 - 900.0) / 100.0 * 2.0
        } else {
            10.0 + (*average as f64 - 850.0) / 50.0 * 4.0
        }
    }

    pub fn explore(
        &mut self,
        maze_n: &usize,
        global_ticks: &mut usize,
        size: &[usize; 2],
        best_timer: &Option<[usize; 2]>,
        average: &usize,
        exec_time: &Instant,
    ) -> Option<usize> {
        let mut t: usize = 0;
        self.display();
        self.tray[self.end[0]][self.end[1]] = indicator::Indicator::PLAYER('🚩');

        loop {
            println!(
                "[Maze '{} {}x{}] - {:.2?}",
                maze_n,
                size[0],
                size[1],
                exec_time.elapsed()
            );

            if let Some(bt) = best_timer {
                println!("# Record steps → {} in [Maze '{}]", bt[0], bt[1]);
            }

            println!("# Total moves → {}", *global_ticks);

            println!(
                "# Average moves → {}",
                if *average > 0 {
                    average.to_string()
                } else {
                    "waiting for more stats...".to_string()
                }
            );

            println!(
                "# Bot score → {}/10",
                if Self::feedback_average(average) > 0.0 {
                    Self::feedback_average(average).to_string()
                } else {
                    "waiting for more stats...".to_string()
                }
            );

            println!("# Moves → {}", t);
            println!(
                "# Direction → {}",
                self.smart_robot.direction.unwrap().get_arrow_from()
            );

            let neighbors = self.smart_robot.get_neighbors(&self.tray);
            println!("# Choice(s) → {:?}\n", neighbors.extract_possible_move());

            self.tray[self.smart_robot.current_case[0]][self.smart_robot.current_case[1]] =
                indicator::Indicator::PLAYER(self.smart_robot.direction.unwrap().get_arrow_from());

            let taken_direction = self.smart_robot.choose_direction(neighbors, false);
            self.smart_robot.set_direction(taken_direction);

            if t == (size[0]*size[1])*10 {
                self.tray[self.smart_robot.current_case[0]][self.smart_robot.current_case[1]] =
                    indicator::Indicator::PLAYER(
                        self.smart_robot.direction.unwrap().get_arrow_from(),
                    );
                self.display();
                return None;
            }

            // Shortcut to help bot (to earn time)
            if [
                self.smart_robot.current_case[0],
                self.smart_robot.current_case[1] + 1,
            ] == self.end
            {
                self.tray[self.smart_robot.current_case[0]][self.smart_robot.current_case[1]] =
                    indicator::Indicator::PLAYER(
                        self.smart_robot.direction.unwrap().get_arrow_from(),
                    );
                self.display();

                return Some(t + 1);
            }

            if self.smart_robot.current_case == self.end {
                self.display();
                return Some(t);
            } else {
                self.display();
                *global_ticks += 1;
                t += 1;
                sleep(Duration::from_millis(2));
                print!("\x1B[2J\x1B[1;1H");
            }
        }
    }
}
