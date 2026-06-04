#[derive(Clone)]
enum Cell {
    Wall,
    Ball,
    Empty,
}
struct Grid {
    play_grid: Vec<Cell>,
    width: usize,
    wall: char,
    ball: char,
}

impl Grid {
    fn get_position_flattened_grid(&self, row: usize, col: usize) -> usize {
        return row * self.width + col;
    }

    fn print_grid(&self) {
        for row in 0..self.play_grid.len() / self.width {
            for col in 0..self.width {
                println!(
                    "Coords: {row},{col} Pos: {}",
                    self.get_position_flattened_grid(row, col)
                );
            }
        }
    }

    fn new(width: usize, height: usize, ball: char, wall: char) -> Self {
        Self {
            play_grid: vec![Cell::Empty; width * height],
            width,
            ball,
            wall,
        }
    }

    fn initialize_walls(&mut self) {}

    fn render(&self) {
        let mut render_string: String = String::from("");
        for row in 0..self.play_grid.len() / self.width {
            for col in 0..self.width {
                let pos = self.get_position_flattened_grid(row, col);
                match self.play_grid[pos] {
                    Cell::Ball => render_string.push_str(&self.ball.to_string()),
                    Cell::Wall => render_string.push_str(&self.wall.to_string()),
                    Cell::Empty => render_string.push_str(" "),
                }
            }
            render_string.push_str("\n");
        }
        print!("{}", render_string);
    }
}

// = vec![Cell::Empty,Cell::Empty,Cell::Empty, Cell::Empty,Cell::Empty,Cell::Empty, Cell::Empty,Cell::Empty,Cell::Empty];
fn main() {
    let grid1 = Grid::new(4, 3, '●', '█');
    grid1.print_grid();
    grid1.render();
}
