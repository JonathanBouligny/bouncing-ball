use std::thread::sleep;
use std::time::Duration;

#[derive(Clone)]
enum Cell {
    Wall,
    Ball,
    Empty,
}

enum Direction {
    Up,
    Down,
    Left,
    Right,
}
struct Grid {
    play_grid: Vec<Cell>,
    width: usize,
    wall: char,
    ball: char,
    ball_row: usize,
    ball_col: usize,
    row_step: isize,
    col_step: isize,
    wait_time: u64,
}

impl Grid {
    fn new(width: usize, height: usize, ball: char, wall: char) -> Result<Self, &'static str> {
        if width > 2 && height > 2 {
            return Ok(Self {
                play_grid: vec![Cell::Empty; width * height],
                width,
                ball,
                wall,
                ball_row: 1,
                ball_col: 1,
                row_step: 1,
                col_step: 1,
                wait_time: 2,
            });
        } else {
            return Err("ERROR: Row and Col must each be larger then 2");
        }
    }

    fn initialize(&mut self) {
        for row in 0..self.get_height() as isize {
            for col in 0..self.width as isize {
                if !self.is_inner_cell(row, col) {
                    self.set_play_grid(row, col, Cell::Wall);
                }
            }
        }

        self.set_play_grid(self.ball_row as isize, self.ball_col as isize, Cell::Ball);
    }

    fn get_position_flattened_grid(&self, row: isize, col: isize) -> isize {
        return row * self.width as isize + col;
    }

    // fn get_coords_grid(&self, idx: isize) -> (isize, isize) {
    //     return (idx / self.width, idx % self.width);
    // }

    fn get_height(&self) -> usize {
        return self.play_grid.len() / self.width;
    }

    fn set_play_grid(&mut self, row: isize, col: isize, cell_type: Cell) {
        let flattened_pos = self.get_position_flattened_grid(row, col);
        self.play_grid[flattened_pos as usize] = cell_type;
    }

    fn set_ball_pos(&mut self, row: isize, col: isize) {
        self.set_play_grid(self.ball_row as isize, self.ball_col as isize, Cell::Empty);
        self.set_play_grid(row, col, Cell::Ball);
        self.ball_row = row as usize;
        self.ball_col = col as usize;
    }

    fn is_inner_cell(&self, row: isize, col: isize) -> bool {
        return row > 0
            && row < self.get_height() as isize - 1
            && col > 0
            && col < self.width as isize - 1;
    }

    fn compute_next_pos(&self) -> (isize, isize) {
        return (
            self.ball_row as isize + self.row_step,
            self.ball_col as isize + self.col_step,
        );
    }

    fn move_ball(&mut self) {
        let (next_row, next_col) = self.compute_next_pos();
        if self.is_inner_cell(next_row, next_col) {
            self.set_ball_pos(next_row, next_col);
        }
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

    fn bounce(&mut self) {}

    fn run(&mut self) {
        while (true) {
            print!("{esc}c", esc = 27 as char);
            self.render();
            sleep(Duration::from_secs(self.wait_time));
            self.move_ball();
            self.bounce();
        }
    }
}

fn main() {
    let grid = Grid::new(10, 10, '●', '█');
    let mut grid = match grid {
        Ok(val) => val,
        Err(val) => {
            println!("{val}");
            return;
        }
    };

    grid.initialize();
    grid.run();
}
