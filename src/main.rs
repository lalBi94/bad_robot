use bad_robot::maze;
use bad_robot::maze::maze_tools;
use std::{
    thread::sleep,
    time::{Duration, Instant},
};

fn main() {
    let mut nb: usize = 1;
    let mut ticks: usize = 0;

    let size: [usize; 2] = [100, 50];

    let mut last_moves_ticks: Vec<usize> = Vec::new();
    let mut best_move_ticks: Option<[usize; 2]> = None;
    let mut average: usize = 0;
    const WITH_DELAY_ON_RESOLVED_MAZE: bool = true;

    let exec_time: Instant = Instant::now();

    loop {
        // You can try with ur own with vec!["0|1", ...]
        let tray: Vec<String> = maze_tools::MazeTools::create_maze(&size[0], &size[1]);

        println!("{:#?}", tray);

        let mut maze: maze::Maze = maze::Maze::new(tray);
        let result: Option<usize> = maze.explore(
            &nb,
            &mut ticks,
            &size,
            &best_move_ticks,
            &average,
            &exec_time,
        );

        if let Some(res) = result {
            last_moves_ticks.push(res);
        }

        if WITH_DELAY_ON_RESOLVED_MAZE {
            if let Some(res) = result {
                println!(
                    "\nThe \"Smart\" Bot found exit in {} steps. (#{} resolved)",
                    res, nb
                );
            } else {
                println!("The robot was unable to complete its mission... (abandoned or path impossible)");
            }
            sleep(Duration::from_secs(3));
            print!("\x1B[2J\x1B[1;1H");
        }

        if let Some(res) = result {
            if let Some(bmt) = best_move_ticks {
                if bmt[0] > res {
                    best_move_ticks = Some([res, nb])
                }
            } else {
                best_move_ticks = Some([res, nb])
            }
        }

        if nb > 5 {
            average = if !last_moves_ticks.is_empty() {
                last_moves_ticks.iter().sum::<usize>() / last_moves_ticks.len()
            } else {
                average
            };
        }

        nb += 1;
    }
}
