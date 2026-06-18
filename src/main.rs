use core::num;
use std::io::{self, Write};
use std::thread::sleep;
use std::time::{Duration, Instant};

#[derive(Clone)]
enum Cell {
    Wall,
    Ball,
    Empty,
}

struct Ball {
    row: usize,
    col: usize,
    row_step: isize,
    col_step: isize,
}

impl Ball {
    fn new(row: usize, col: usize, row_step: isize, col_step: isize) -> Self {
        Self {
            row,
            col,
            row_step,
            col_step,
        }
    }

    fn compute_next_pos(&self) -> (isize, isize) {
        return (
            self.row as isize + self.row_step,
            self.col as isize + self.col_step,
        );
    }

    fn move_ball(&mut self, width: usize, height: usize) {
        let (next_row, next_col) = self.compute_next_pos();
        if next_row > 0
            && next_row < height as isize - 1
            && next_col > 0
            && next_col < width as isize - 1
        {
            self.row = next_row as usize;
            self.col = next_col as usize;
        } else {
            // Bounce
            if next_col <= 0 || next_col >= width as isize - 1 {
                //flip from - to + movement direction on the x axis
                self.col_step *= -1;
            }

            if next_row <= 0 || next_row >= height as isize - 1 {
                // flip from from - to + on the y axis
                self.row_step *= -1;
            }
        }
    }
}

struct Grid {
    play_grid: Vec<Cell>,
    width: usize,
    wall: char,
    ball: char,
    wait_time: u64,
    buf_writer: io::BufWriter<io::Stdout>,
    terminated: bool,
    balls: Vec<Ball>,
    run_time: u64,
}

impl Grid {
    fn new(
        width: usize,
        height: usize,
        ball: char,
        wall: char,
        num_balls: usize,
        run_time: u64,
    ) -> Result<Self, &'static str> {
        if width > 2 && height > 2 {
            let mut balls: Vec<Ball> = Vec::new();
            let mut row = 1;
            let mut col = 1;
            for n in 0..num_balls {
                balls.push(Ball::new(row, col, 1, 1));
                row = row + 1 % height;
                if row == height - 1 {
                    col = col + 1 % width;
                }
            }

            return Ok(Self {
                play_grid: vec![Cell::Empty; width * height],
                balls,
                width,
                ball,
                wall,
                wait_time: 33,
                buf_writer: io::BufWriter::new(io::stdout()),
                terminated: false,
                run_time,
            });
        } else {
            return Err("ERROR: Row and Col must each be larger then 2");
        }
    }

    fn initialize(&mut self) -> io::Result<()> {
        for row in 0..self.get_height() as isize {
            for col in 0..self.width as isize {
                if !self.is_inner_cell(row, col) {
                    self.set_play_grid(row, col, Cell::Wall);
                }
            }
        }

        // enable alternative screen buffer
        write!(self.buf_writer, "\x1b[?1049h")?;
        // hide cursor
        write!(self.buf_writer, "\x1b[?25l")?;

        Ok(())
    }

    fn terminate(&mut self) -> io::Result<()> {
        // show cursor
        write!(self.buf_writer, "\x1b[?25h")?;
        // disable alternative screen buffer
        write!(self.buf_writer, "\x1b[?1049l")?;

        // BufferWriter flushes on its own drop on the way out so no flush is needed here
        self.buf_writer.flush()?;

        self.terminated = true;
        Ok(())
    }

    fn get_position_flattened_grid(&self, row: isize, col: isize) -> isize {
        return row * self.width as isize + col;
    }

    fn get_height(&self) -> usize {
        return self.play_grid.len() / self.width;
    }

    fn set_play_grid(&mut self, row: isize, col: isize, cell_type: Cell) {
        let flattened_pos = self.get_position_flattened_grid(row, col);
        self.play_grid[flattened_pos as usize] = cell_type;
    }

    fn is_inner_cell(&self, row: isize, col: isize) -> bool {
        return row > 0
            && row < self.get_height() as isize - 1
            && col > 0
            && col < self.width as isize - 1;
    }

    fn render(&mut self) -> io::Result<()> {
        write!(self.buf_writer, "\x1b[H")?;

        for row in 0..self.get_height() {
            for col in 0..self.width {
                let ball_in_cell = self
                    .balls
                    .iter()
                    .any(|ball| ball.row == row && ball.col == col);

                if !self.is_inner_cell(row as isize, col as isize) {
                    write!(self.buf_writer, "{}{}", self.wall, self.wall)?
                } else if ball_in_cell {
                    write!(self.buf_writer, "{} ", self.ball)?
                } else {
                    write!(self.buf_writer, "  ")?
                }
            }
            write!(self.buf_writer, "\n")?
        }

        self.buf_writer.flush()?;

        Ok(())
    }

    fn run(&mut self) -> io::Result<()> {
        let start = Instant::now();
        while start.elapsed() < Duration::from_secs(self.run_time) {
            self.render()?;
            sleep(Duration::from_millis(self.wait_time));
            let width1 = self.width;
            let height2 = self.get_height();
            self.balls.iter_mut().for_each(|ball| {
                ball.move_ball(width1, height2);
            });
        }

        Ok(())
    }
}

impl Drop for Grid {
    fn drop(&mut self) {
        if !self.terminated
            && let Err(e) = self.terminate()
        {
            println!("{e}");
        }
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut grid = Grid::new(20, 10, '●', '█', 4, 5)?;
    grid.initialize()?;
    grid.run()?;
    grid.terminate()?;

    Ok(())
}
