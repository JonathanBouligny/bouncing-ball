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
    ball_x: usize,
    ball_y: usize,
}

impl Grid {
    fn new(width: usize, height: usize, ball: char, wall: char) -> Self {
        Self {
            play_grid: vec![Cell::Empty; width * height],
            width,
            ball,
            wall,
            ball_x: 0,
            ball_y: 0,
        }
    }

    fn get_position_flattened_grid(&self, row: usize, col: usize) -> usize {
        return row * self.width + col;
    }

    fn get_coords_grid(&self, idx: usize) -> (usize, usize) {
        return (idx / self.width, idx % self.width);
    }

    fn print_grid(&self) {
        // for row in 0..self.get_height() {
        //     for col in 0..self.width {}
        // }

        self.play_grid.iter().enumerate().for_each(|(idx, Cell)| {
            let (row, col) = self.get_coords_grid(idx);
            println!("Coords: {row},{col} Pos: {idx}",);
        });
    }

    fn get_height(&self) -> usize {
        return self.play_grid.len() / self.width;
    }

    fn is_inner_cell(&self, row: usize, col: usize) -> bool {
        return row > 0 && row < self.get_height() - 1 && col > 0 && col < self.width - 1;
    }
    fn initialize_walls(&mut self) {
        for row in 0..self.get_height() {
            for col in 0..self.width {
                if !self.is_inner_cell(row, col) {
                    let coord = self.get_position_flattened_grid(row, col);
                    self.play_grid[coord] = Cell::Wall;
                }
            }
        }
    }

    fn initialize_ball(&mut self) {}

    fn render(&self) {
        let mut render_string: String = String::from("");
        // for row in 0..self.get_height() {
        //     for col in 0..self.width {
        //         let pos = self.get_position_flattened_grid(row, col);
        //         match self.play_grid[pos] {
        //             Cell::Ball => render_string.push_str(&self.ball.to_string()),
        //             Cell::Wall => render_string.push_str(&self.wall.to_string()),
        //             Cell::Empty => render_string.push_str(" "),
        //         }
        //     }
        //     render_string.push_str("\n");
        // }
        let render_loop = |(idx, cell)| {
            match self.play_grid[idx] {
                Cell::Ball => render_string.push_str(&self.ball.to_string()),
                Cell::Wall => render_string.push_str(&self.wall.to_string()),
                Cell::Empty => render_string.push_str(" "),
            }
            if (idx + 1) % self.width == 0 {
                render_string.push_str("\n");
            }
        };
        self.play_grid.iter().enumerate().for_each(render_loop);
        print!("{}", render_string);
    }
}

// = vec![Cell::Empty,Cell::Empty,Cell::Empty, Cell::Empty,Cell::Empty,Cell::Empty, Cell::Empty,Cell::Empty,Cell::Empty];
fn main() {
    let mut grid1 = Grid::new(10, 10, '●', '█');
    grid1.initialize_walls();
    grid1.initialize_ball();
    grid1.print_grid();
    grid1.render();
}
