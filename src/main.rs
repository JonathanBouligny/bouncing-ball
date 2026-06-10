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
    ball_row: usize,
    ball_col: usize,
}

impl Grid {
    fn new(width: usize, height: usize, ball: char, wall: char) -> Result<Self, &'static str> {
        if width > 2 && height > 2 {
            return Ok(Self {
                play_grid: vec![Cell::Empty; width * height],
                width,
                ball,
                wall,
                ball_row: 0,
                ball_col: 0,
            });
        } else {
            return Err("ERROR: Row and Col must each be larger then 2");
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

    fn initialize_ball(&mut self) {
        // let mut rng = rand::rng();
        // let mut row = 0;
        // let mut col = 0;
        // let mut idx = 0;
        // while !self.is_inner_cell(row, col) {
        //     idx = rng.random_range(..self.play_grid.len());
        //     (row, col) = self.get_coords_grid(idx);
        // }
        let flattened_pos = self.width + 1;
        self.set_ball_pos(flattened_pos);
    }

    fn set_ball_pos(&mut self, flattened_pos: usize) {
        self.play_grid[flattened_pos] = Cell::Ball;
        (self.ball_row, self.ball_col) = self.get_coords_grid(flattened_pos);
    }

    fn render(&self) {
        let mut render_string: String = String::from("");
        let render_closure = |(idx, cell): (usize, &Cell)| {
            match cell {
                &Cell::Ball => render_string.push_str(&self.ball.to_string()),
                &Cell::Wall => render_string.push_str(&self.wall.to_string()),
                &Cell::Empty => render_string.push_str(" "),
            }
            if (idx + 1) % self.width == 0 {
                render_string.push_str("\n");
            }
        };
        self.play_grid.iter().enumerate().for_each(render_closure);
        print!("{}", render_string);
    }
}

// = vec![Cell::Empty,Cell::Empty,Cell::Empty, Cell::Empty,Cell::Empty,Cell::Empty, Cell::Empty,Cell::Empty,Cell::Empty];
fn main() {
    let grid = Grid::new(10, 10, '●', '█');
    let mut grid = match grid {
        Ok(val) => val,
        Err(val) => {
            println!("{val}");
            return;
        }
    };
    grid.initialize_walls();
    grid.initialize_ball();
    grid.print_grid();
    grid.render();
}
