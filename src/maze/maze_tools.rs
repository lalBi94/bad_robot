use crate::indicator;
use crate::smart_robot::smart_robot_move;
use rand::{thread_rng, Rng};

pub struct MazeTools {}

impl MazeTools {
    fn create_blank_maze(width: &usize, height: &usize) -> Vec<String> {
        const WALL: char = '0';
        const FREE: char = '1';

        let mut maze: Vec<String> = Vec::new();

        let mut rng: rand::prelude::ThreadRng = rand::thread_rng();
        let end: usize = rng.gen_range(1..(height / 2) + 1);
        let start: usize = rng.gen_range(1..(height / 2) - 1);

        for i in 0..height - 1 {
            if i % 2 == 0 {
                let mut str: String = String::new();
                str.push_str(&String::from(WALL).repeat(*width));

                maze.push(str);
            } else {
                let mut str: String = String::new();

                for j in 0..*width {
                    str.push(if j % 2 == 0 { WALL } else { FREE });
                }

                maze.push(str);
            }
        }

        maze[start].replace_range(0..2, "11");
        maze[end].replace_range(width - 2..*width, "11");

        maze
    }
    pub fn create_maze(width: &usize, height: &usize) -> Vec<String> {
        let check_width: usize = if *width % 2 == 0 { *width + 1 } else { *width };

        let mut blank: Vec<String> = Self::create_blank_maze(&check_width, height);
        let mut last_blank: Vec<String> = Vec::new();
        let mut redondance: usize = 0;

        for row in 0..blank.len() {
            for col in 0..blank[row].len() {
                if blank[row].as_bytes()[col] == b'1' {
                    let mut rng = rand::thread_rng();
                    let random_value: usize = if rng.gen_bool(0.2) {
                        5
                    } else {
                        rng.gen_range(2..=6)
                    };
                    blank[row].replace_range(col..col + 1, &random_value.to_string());
                }
            }
        }

        loop {
            let mut rng_ri: rand::prelude::ThreadRng = rand::thread_rng();
            let mut random_index_row: usize = rng_ri.gen_range(0..*width);

            if random_index_row % 2 == 0 {
                while random_index_row % 2 == 0 {
                    random_index_row = rng_ri.gen_range(0..*width)
                }
            }

            let mut rng_ric: rand::prelude::ThreadRng = rand::thread_rng();
            let mut random_index_col: usize = rng_ric.gen_range(0..*height - 1);

            if random_index_col % 2 == 0 {
                while random_index_col % 2 == 0 {
                    random_index_col = rng_ric.gen_range(0..*height - 1)
                }
            }

            if last_blank.eq(&blank) {
                if redondance == 400 {
                    break;
                } else {
                    redondance += 1
                }
            } else {
                last_blank = blank.clone();
            }

            let mut neighbors: Vec<([usize; 2], smart_robot_move::SmartRobotMove)> = Vec::new();

            match blank.get(random_index_col) {
                Some(val) => {
                    if random_index_col >= 2 {
                        if val.as_bytes().get(random_index_row) != Some(&b'0') {
                            if let Some(top_val) = blank.get(random_index_col - 2) {
                                if top_val.as_bytes().get(random_index_row) != Some(&b'0') {
                                    neighbors.push((
                                        [random_index_col - 2, random_index_row],
                                        smart_robot_move::SmartRobotMove::TOP,
                                    ))
                                }
                            }
                        }
                    }
                }
                None => (),
            }

            match blank.get(random_index_col) {
                Some(_) => match blank[random_index_col].as_bytes().get(random_index_row + 2) {
                    Some(v) => {
                        if *v != b'0' {
                            neighbors.push((
                                [random_index_col, random_index_row + 2],
                                smart_robot_move::SmartRobotMove::RIGHT,
                            ))
                        }
                    }
                    None => (),
                },
                _ => (),
            };

            match blank.get(random_index_col) {
                Some(_) => match blank[random_index_col].as_bytes().get(random_index_row) {
                    Some(v) => {
                        if random_index_row >= 2 && *v != b'0' {
                            if blank[random_index_col]
                                .as_bytes()
                                .get(random_index_row - 2)
                                .unwrap_or(&b'0')
                                != &b'0'
                            {
                                neighbors.push((
                                    [random_index_col, random_index_row - 2],
                                    smart_robot_move::SmartRobotMove::LEFT,
                                ))
                            }
                        }
                    }
                    None => (),
                },
                _ => (),
            };

            match blank.get(random_index_col + 2) {
                Some(_) => {
                    if blank[random_index_col + 2].as_bytes()[random_index_row] != b'0' {
                        neighbors.push((
                            [random_index_col + 2, random_index_row],
                            smart_robot_move::SmartRobotMove::DOWN,
                        ))
                    }
                }
                _ => (),
            };

            let mut rng_rn: rand::prelude::ThreadRng = thread_rng();
            let random_neighbors: usize = rng_rn.gen_range(0..neighbors.len());

            let current_num_focus_num: char =
                std::char::from_u32(blank[random_index_col].as_bytes()[random_index_row].into())
                    .unwrap();
            let targeted_neighbors_num: char = std::char::from_u32(
                blank[neighbors[random_neighbors].0[0]].as_bytes()[neighbors[random_neighbors].0[1]]
                    as u32,
            )
            .unwrap();

            blank[random_index_col].replace_range(
                random_index_row..random_index_row + 1,
                targeted_neighbors_num.to_string().as_str(),
            );

            if current_num_focus_num != targeted_neighbors_num {
                match neighbors[random_neighbors].1 {
                    smart_robot_move::SmartRobotMove::TOP => {
                        blank[neighbors[random_neighbors].0[0] + 1].replace_range(
                            neighbors[random_neighbors].0[1]..neighbors[random_neighbors].0[1] + 1,
                            targeted_neighbors_num.to_string().as_str(),
                        )
                    }

                    smart_robot_move::SmartRobotMove::DOWN => {
                        blank[neighbors[random_neighbors].0[0] - 1].replace_range(
                            neighbors[random_neighbors].0[1]..neighbors[random_neighbors].0[1] + 1,
                            targeted_neighbors_num.to_string().as_str(),
                        )
                    }

                    smart_robot_move::SmartRobotMove::LEFT => {
                        blank[neighbors[random_neighbors].0[0]].replace_range(
                            neighbors[random_neighbors].0[1] + 1
                                ..neighbors[random_neighbors].0[1] + 2,
                            targeted_neighbors_num.to_string().as_str(),
                        )
                    }

                    smart_robot_move::SmartRobotMove::RIGHT => {
                        blank[neighbors[random_neighbors].0[0]].replace_range(
                            neighbors[random_neighbors].0[1] - 2
                                ..neighbors[random_neighbors].0[1] - 1,
                            targeted_neighbors_num.to_string().as_str(),
                        )
                    }
                }
            }

            let display_blank = || {
                for e in blank.iter() {
                    for i in e.chars() {
                        print!(
                            "{}",
                            indicator::Indicator::char_to_indicator(i).representation_of()
                        )
                    }

                    println!();
                }
            };

            println!("Maze loading\n");
            println!("{:#?}", display_blank());
            print!("\x1B[2J\x1B[1;1H");
        }

        blank
    }
}
