use std::io::{self, Write};
use std::thread::sleep;
use std::time::{Duration, Instant};

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
    row_step: isize,
    col_step: isize,
    wait_time: u64,
    buf_writer: io::BufWriter<io::Stdout>,
    terminated: bool,
}

struct Ball {
    row: usize,
    col: usize,
    row_step: isize,
    col_step: isize,
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
                wait_time: 33,
                buf_writer: io::BufWriter::new(io::stdout()),
                terminated: false,
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

        // stamp balls
        self.set_play_grid(self.ball_row as isize, self.ball_col as isize, Cell::Ball);
        //
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
        } else {
            self.bounce(next_row, next_col);
        }
    }

    fn bounce(&mut self, row: isize, col: isize) {
        if col <= 0 || col >= self.width as isize - 1 {
            //flip from - to + movement direction on the x axis
            self.col_step *= -1;
        }

        if row <= 0 || row >= self.get_height() as isize - 1 {
            // flip from from - to + on the y axis
            self.row_step *= -1;
        }
    }

    fn render(&mut self) -> io::Result<()> {
        write!(self.buf_writer, "\x1b[H")?;

        for (idx, cell) in self.play_grid.iter().enumerate() {
            match cell {
                &Cell::Ball => write!(self.buf_writer, "{} ", self.ball)?,
                &Cell::Wall => write!(self.buf_writer, "{}{}", self.wall, self.wall)?,
                &Cell::Empty => write!(self.buf_writer, "  ")?,
            }
            if (idx + 1) % self.width == 0 && idx != self.play_grid.len() - 1 {
                write!(self.buf_writer, "\n")?
            }
        }

        self.buf_writer.flush()?;

        Ok(())
    }

    fn clear_inner_cells(&mut self) {
        for row in 1..self.get_height() as isize - 1 {
            for col in 1..self.width as isize - 1 {
                self.set_play_grid(row, col, Cell::Empty);
            }
        }
    }

    fn run(&mut self) -> io::Result<()> {
        let start = Instant::now();
        while start.elapsed() < Duration::from_secs(5) {
            self.render()?;
            sleep(Duration::from_millis(self.wait_time));
            self.clear_inner_cells();
            // balls calculate where they need to be
            // stamp balls
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
    let mut grid = Grid::new(20, 10, '●', '█')?;
    grid.initialize()?;
    grid.run()?;
    grid.terminate()?;

    Ok(())
}
