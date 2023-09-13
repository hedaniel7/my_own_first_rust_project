

#[derive(Clone, PartialEq)]
enum Cell {
    Empty,
    Obstacle,
    Start,
    Goal,
    Path,
}

struct Robot {
    position: (usize, usize),
}


impl Robot {
    fn move_right(&mut self, grid: &mut Grid) {
        let (row, col) = self.position;
        
        // Check if we can move right and if the next cell is not an obstacle
        if col < GRID_SIZE - 1 && grid[row][col + 1] != Cell::Obstacle {
            // Update the current cell to Empty and the next cell to Path
            grid[row][col] = Cell::Empty;
            grid[row][col + 1] = Cell::Path;

            // Update robot's position
            self.position = (row, col + 1);
        }
    }
}



const GRID_SIZE: usize = 10; // A 10x10 grid for simplicity

type Grid = Vec<Vec<Cell>>;

fn init_grid() -> Grid {
    let mut grid = vec![vec![Cell::Empty; GRID_SIZE]; GRID_SIZE];
    
    // For now, let's place a start and goal
    grid[0][0] = Cell::Start;
    grid[GRID_SIZE-1][GRID_SIZE-1] = Cell::Goal;

    grid
}

fn display_grid(grid: &Grid) {
    for row in grid.iter() {
        for cell in row.iter() {
            match cell {
                Cell::Empty => print!(". "),
                Cell::Obstacle => print!("X "),
                Cell::Start => print!("S "),
                Cell::Goal => print!("G "),
                Cell::Path => print!("* "),
            }
        }
        println!();
    }
}



fn main() {
    let mut grid = init_grid();
    let mut robot = Robot { position: (0, 0) };

    robot.move_right(&mut grid);

    display_grid(&grid);

    robot.move_right(&mut grid);

    display_grid(&grid);
}
