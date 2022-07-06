use crate::MonstrousMazeInput;

pub struct Grid {
    pub grid: Vec<String>,
    pub start: (u64, u64),
    pub end: (u64, u64),
    pub endurance: u8,
}

impl Grid {
    pub fn new(input: MonstrousMazeInput) -> Grid {
        let split_grid = input.grid.split("\n").collect::<Vec<&str>>().iter().map(|line| line.to_string()).collect::<Vec<String>>();
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
pub fn get_best_path(filtered_possible_solutions: Vec<&GridPossibleSolution>) -> &GridPossibleSolution {
    return filtered_possible_solutions
        .iter()
        .min_by(|a, b| {
            if a.endurance_left == b.endurance_left {
                return a.path_taken.len().cmp(&b.path_taken.len());
            }
            return a.endurance_left.cmp(&b.endurance_left);
        })
        .unwrap()
}

pub fn find_paths(grid: &mut Grid, mut grid_possible_solution: GridPossibleSolution) -> Vec<GridPossibleSolution> {
    if grid_possible_solution.visited_coordinates.contains(&grid_possible_solution.current_coordinates) {
        return vec![];
    }
    if grid_possible_solution.endurance_left <= 0 {
        return vec![];
    }

    grid_possible_solution.visited_coordinates.push(grid_possible_solution.current_coordinates);

    let mut paths: Vec<GridPossibleSolution> = vec![];

    let current_line: String = grid.grid[grid_possible_solution.current_coordinates.0 as usize].clone();
    let current_char: char = current_line.chars().nth(grid_possible_solution.current_coordinates.1 as usize).unwrap() as char;

    return if current_char == START_CHARACTER || current_char == END_CHARACTER || current_char == MONSTER_CHARACTER || current_char == FREE_WAY_CHARACTER {
        if current_char == END_CHARACTER {
            grid_possible_solution.success = true;
            paths.push(grid_possible_solution);
            return paths;
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

        all_paths
    } else {
        paths
    }
}

fn go_to_left(
    all_paths: &mut Vec<GridPossibleSolution>,
    grid_possible_solution: &GridPossibleSolution,
    grid: &mut Grid,
) {
    let left_direction = '<';
    let left_coordinates = (grid_possible_solution.current_coordinates.0, grid_possible_solution.current_coordinates.1 - 1);
    move_in_maze(left_direction, left_coordinates, grid_possible_solution, grid, all_paths);
}

fn go_to_top(
    all_paths: &mut Vec<GridPossibleSolution>,
    grid_possible_solution: &GridPossibleSolution,
    grid: &mut Grid,
) {
    let top_direction = '^';
    let top_coordinates = (grid_possible_solution.current_coordinates.0 - 1, grid_possible_solution.current_coordinates.1);
    move_in_maze(top_direction, top_coordinates, grid_possible_solution, grid, all_paths);
}

fn go_to_right(
    all_paths: &mut Vec<GridPossibleSolution>,
    grid_possible_solution: &GridPossibleSolution,
    grid: &mut Grid,
) {
    let right_direction = '>';
    let right_coordinates = (grid_possible_solution.current_coordinates.0, grid_possible_solution.current_coordinates.1 + 1);
    move_in_maze(right_direction, right_coordinates, grid_possible_solution, grid, all_paths);
}

fn go_to_bottom(
    all_paths: &mut Vec<GridPossibleSolution>,
    grid_possible_solution: &GridPossibleSolution,
    grid: &mut Grid,
) {
    let bottom_direction = 'v';
    let bottom_coordinates = (grid_possible_solution.current_coordinates.0 + 1, grid_possible_solution.current_coordinates.1);
    move_in_maze(bottom_direction, bottom_coordinates, grid_possible_solution, grid, all_paths);
}

fn move_in_maze(
    direction: char,
    new_coordinates: (i64, i64),
    grid_possible_solution: &GridPossibleSolution,
    grid: &mut Grid,
    all_paths: &mut Vec<GridPossibleSolution>) {
    if is_coordinates_in_grid(new_coordinates, grid) {
        let new_grid_possible_solution = GridPossibleSolution {
            current_coordinates: new_coordinates,
            path_taken: format!("{}{}", grid_possible_solution.path_taken.clone(), direction),
            visited_coordinates: grid_possible_solution.visited_coordinates.clone(),
            success: false,
            endurance_left: grid_possible_solution.endurance_left,
        };
        all_paths.append(&mut find_paths(grid, new_grid_possible_solution));
    }
}

fn is_coordinates_in_grid(coordinates: (i64, i64), grid: &Grid) -> bool {
    let (line_index, column_index) = coordinates;
    let line_count = grid.grid.len();
    let column_count = grid.grid[0].len();
    return line_index < line_count as i64 && column_index < column_count as i64 && line_index >= 0 && column_index >= 0;
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
