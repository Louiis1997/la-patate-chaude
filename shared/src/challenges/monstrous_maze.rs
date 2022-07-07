use crate::challenges::Challenge;
use crate::{MonstrousMazeInput, MonstrousMazeOutput};

#[derive(Debug, Clone)]
pub struct MonstrousMaze {
    pub input: MonstrousMazeInput,
}

impl Challenge for MonstrousMaze {
    type Input = MonstrousMazeInput;
    type Output = MonstrousMazeOutput;

    fn name() -> String {
        return "MonstrousMaze".to_string();
    }

    fn new(input: Self::Input) -> Self {
        return Self { input };
    }

    fn solve(&self) -> Self::Output {
        let mut final_output = MonstrousMazeOutput {
            path: "".to_string(),
        };

        let mut grid: Grid = Grid::new(self.input.clone());
        println!("Grid start: {:?}", grid.start);
        println!("Grid end: {:?}", grid.end);

        let grid_possible_solution: GridPossibleSolution = GridPossibleSolution {
            path_taken: "".to_string(),
            current_coordinates: (grid.start.0 as i64, grid.start.1 as i64),
            visited_coordinates: vec![],
            success: false,
            endurance_left: grid.endurance as i8,
        };

        let possible_solutions = find_paths(&mut grid, grid_possible_solution);
        match possible_solutions {
            Some(solutions) => {
                let no_solution_because_died = solutions
                    .iter()
                    .all(|solution| solution.endurance_left <= 0);
                if no_solution_because_died {
                    println!("/!\\ No solution found because '☠️ YOU DIED ☠️' /!\\");
                    return final_output;
                }

                // Filter successful & not empty paths
                let successful_paths: Vec<&GridPossibleSolution> = solutions
                    .iter()
                    .filter(|path| path.success && !path.path_taken.is_empty())
                    .collect::<Vec<&GridPossibleSolution>>();

                if successful_paths.len() == 0 {
                    println!("/!\\ No solution because no path found in Monstrous Maze ☹️ /!\\");
                    return final_output;
                }

                // Display found paths
                // println!("Found paths:");
                // for path in &successful_paths {
                //     println!("path {:?} - endurance: {} - success: {}", &path.path_taken, &path.endurance_left, &path.success);
                // }

                match get_best_path(successful_paths) {
                    Some(best_path) => {
                        final_output.path = best_path.path_taken.clone();
                    }
                    None => {
                        println!("/!\\ No solution because no path found in Monstrous Maze ☹️ /!\\");
                    }
                }

                return final_output;
            }
            None => {
                panic!("No possible solution found");
            }
        }
    }

    fn verify(&self, answer: &Self::Output) -> bool {
        if answer.path.is_empty() {
            return false;
        }
        let mut endurance_left = self.input.endurance as i8;
        let grid: Grid = Grid::new(self.input.clone());
        let mut current_coordinates = (grid.start.0 as i64, grid.start.1 as i64);

        for character in answer.path.chars() {
            let next_coordinates = match character {
                '^' => (current_coordinates.0, current_coordinates.1 - 1),
                'V' => (current_coordinates.0, current_coordinates.1 + 1),
                '<' => (current_coordinates.0 - 1, current_coordinates.1),
                '>' => (current_coordinates.0 + 1, current_coordinates.1),
                _ => panic!("Invalid character in path"),
            };
            if is_coordinates_in_grid(next_coordinates, &grid) {
                current_coordinates = next_coordinates;
                let current_line: String = grid.grid[current_coordinates.0 as usize].clone();
                let current_char: char = current_line
                    .chars()
                    .nth(current_coordinates.1 as usize)
                    .unwrap() as char;
                if current_char == MONSTER_CHARACTER {
                    endurance_left -= 1;
                } else if current_char == END_CHARACTER
                    && current_coordinates == (grid.end.0 as i64, grid.end.1 as i64)
                {
                    return endurance_left > 0;
                } else if current_char != ' ' {
                    return false;
                }
            } else {
                return false;
            }
        }

        return true;
    }
}

pub struct Grid {
    pub grid: Vec<String>,
    pub start: (u64, u64),
    pub end: (u64, u64),
    pub endurance: u8,
}

impl Grid {
    pub fn new(input: MonstrousMazeInput) -> Grid {
        let split_grid = input
            .grid
            .split("\n")
            .collect::<Vec<&str>>()
            .iter()
            .map(|line| line.to_string())
            .collect::<Vec<String>>();
        let grid_start = Grid::find_start_coordinates(&split_grid);
        let grid_end = Grid::find_end_coordinates(&split_grid);

        Grid {
            grid: split_grid,
            start: grid_start,
            end: grid_end,
            endurance: input.endurance,
        }
    }

    fn find_coordinates_by_char(grid: &Vec<String>, char_to_find: char) -> (u64, u64) {
        let mut start_coordinates = (0, 0);
        for (line_index, line) in grid.iter().enumerate() {
            for (column_index, column) in line.chars().enumerate() {
                if column == char_to_find {
                    start_coordinates = (line_index as u64, column_index as u64);
                }
            }
        }
        return start_coordinates;
    }

    fn find_start_coordinates(split_grid: &Vec<String>) -> (u64, u64) {
        Grid::find_coordinates_by_char(split_grid, START_CHARACTER)
    }

    fn find_end_coordinates(split_grid: &Vec<String>) -> (u64, u64) {
        Grid::find_coordinates_by_char(split_grid, END_CHARACTER)
    }
}

pub struct GridPossibleSolution {
    pub current_coordinates: (i64, i64),
    pub path_taken: String,
    pub visited_coordinates: Vec<(i64, i64)>,
    pub success: bool,
    pub endurance_left: i8,
}

const START_CHARACTER: char = 'I';
const END_CHARACTER: char = 'X';
const MONSTER_CHARACTER: char = 'M';
const FREE_WAY_CHARACTER: char = ' ';

/// Get best path by used endurance and path length
pub fn get_best_path(
    filtered_possible_solutions: Vec<&GridPossibleSolution>,
) -> Option<&GridPossibleSolution> {
    let solution = filtered_possible_solutions.iter().min_by(|a, b| {
        if a.endurance_left == b.endurance_left {
            return a.path_taken.len().cmp(&b.path_taken.len());
        }
        return a.endurance_left.cmp(&b.endurance_left);
    })?;
    Some(solution)
}

pub fn find_paths(
    grid: &mut Grid,
    mut grid_possible_solution: GridPossibleSolution,
) -> Option<Vec<GridPossibleSolution>> {
    if grid_possible_solution
        .visited_coordinates
        .contains(&grid_possible_solution.current_coordinates)
    {
        return Some(vec![]);
    }
    if grid_possible_solution.endurance_left <= 0 {
        return Some(vec![]);
    }

    grid_possible_solution
        .visited_coordinates
        .push(grid_possible_solution.current_coordinates);

    let mut paths: Vec<GridPossibleSolution> = vec![];

    let current_line: String =
        grid.grid[grid_possible_solution.current_coordinates.0 as usize].clone();
    let current_char: char = current_line
        .chars()
        .nth(grid_possible_solution.current_coordinates.1 as usize)?
        as char;

    if current_char == START_CHARACTER
        || current_char == END_CHARACTER
        || current_char == MONSTER_CHARACTER
        || current_char == FREE_WAY_CHARACTER
    {
        if current_char == END_CHARACTER {
            grid_possible_solution.success = true;
            paths.push(grid_possible_solution);
            return Some(paths);
        }

        if current_char == MONSTER_CHARACTER {
            // println!("endurance_left: {}", grid.endurance);
            grid_possible_solution.endurance_left -= 1;
        }

        let mut all_paths: Vec<GridPossibleSolution> = vec![];

        go_to_right(&mut all_paths, &grid_possible_solution, grid);
        go_to_top(&mut all_paths, &grid_possible_solution, grid);
        go_to_left(&mut all_paths, &grid_possible_solution, grid);
        go_to_bottom(&mut all_paths, &grid_possible_solution, grid);

        Some(all_paths)
    } else {
        Some(paths)
    }
}

fn go_to_left(
    all_paths: &mut Vec<GridPossibleSolution>,
    grid_possible_solution: &GridPossibleSolution,
    grid: &mut Grid,
) {
    let left_direction = '<';
    let left_coordinates = (
        grid_possible_solution.current_coordinates.0,
        grid_possible_solution.current_coordinates.1 - 1,
    );
    move_in_maze(
        left_direction,
        left_coordinates,
        grid_possible_solution,
        grid,
        all_paths,
    );
}

fn go_to_top(
    all_paths: &mut Vec<GridPossibleSolution>,
    grid_possible_solution: &GridPossibleSolution,
    grid: &mut Grid,
) {
    let top_direction = '^';
    let top_coordinates = (
        grid_possible_solution.current_coordinates.0 - 1,
        grid_possible_solution.current_coordinates.1,
    );
    move_in_maze(
        top_direction,
        top_coordinates,
        grid_possible_solution,
        grid,
        all_paths,
    );
}

fn go_to_right(
    all_paths: &mut Vec<GridPossibleSolution>,
    grid_possible_solution: &GridPossibleSolution,
    grid: &mut Grid,
) {
    let right_direction = '>';
    let right_coordinates = (
        grid_possible_solution.current_coordinates.0,
        grid_possible_solution.current_coordinates.1 + 1,
    );
    move_in_maze(
        right_direction,
        right_coordinates,
        grid_possible_solution,
        grid,
        all_paths,
    );
}

fn go_to_bottom(
    all_paths: &mut Vec<GridPossibleSolution>,
    grid_possible_solution: &GridPossibleSolution,
    grid: &mut Grid,
) {
    let bottom_direction = 'v';
    let bottom_coordinates = (
        grid_possible_solution.current_coordinates.0 + 1,
        grid_possible_solution.current_coordinates.1,
    );
    move_in_maze(
        bottom_direction,
        bottom_coordinates,
        grid_possible_solution,
        grid,
        all_paths,
    );
}

fn move_in_maze(
    direction: char,
    new_coordinates: (i64, i64),
    grid_possible_solution: &GridPossibleSolution,
    grid: &mut Grid,
    all_paths: &mut Vec<GridPossibleSolution>,
) {
    if is_coordinates_in_grid(new_coordinates, grid) {
        let new_grid_possible_solution = GridPossibleSolution {
            current_coordinates: new_coordinates,
            path_taken: format!("{}{}", grid_possible_solution.path_taken.clone(), direction),
            visited_coordinates: grid_possible_solution.visited_coordinates.clone(),
            success: false,
            endurance_left: grid_possible_solution.endurance_left,
        };
        match find_paths(grid, new_grid_possible_solution) {
            Some(mut paths) => {
                all_paths.append(&mut paths);
            }
            None => {
                // println!("No path found");
            }
        }
    }
}

fn is_coordinates_in_grid(coordinates: (i64, i64), grid: &Grid) -> bool {
    let (line_index, column_index) = coordinates;
    let line_count = grid.grid.len();
    let column_count = grid.grid[0].len();
    return line_index < line_count as i64
        && column_index < column_count as i64
        && line_index >= 0
        && column_index >= 0;
}

#[cfg(test)]
mod monstrous_maze_tests {
    use crate::challenges::monstrous_maze::MonstrousMaze;
    use crate::challenges::Challenge;
    use crate::MonstrousMazeInput;

    #[test]
    fn monstrous_maze_challenge() {
        let monstrous_maze_input: MonstrousMazeInput = MonstrousMazeInput {
            endurance: 10,
            grid: "|I|\n\
                 | |\n\
                 | |\n\
                 |X|\n"
                .to_string(),
        };
        let monstrous_maze_challenge = MonstrousMaze::new(monstrous_maze_input);
        let expected_path = "vvv".to_string();
        let output = monstrous_maze_challenge.solve();
        let found_path = output.path;

        assert_eq!(found_path, expected_path);
    }
}
