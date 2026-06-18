use std::io::{self, Write};
use std::thread::sleep;
use std::time::{Duration, Instant};

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

    fn move_ball(&mut self, width: usize, height: usize) {
        let next_row = self.row as isize + self.row_step;
        let next_col = self.col as isize + self.col_step;

        // we have to use isize here because the comparison doesnt let us do usize to isize
        if next_row > 0
            && next_row < height as isize - 1
            && next_col > 0
            && next_col < width as isize - 1
        {
            // if these values are inbounds its safe to convert this back to a usize
            self.row = next_row as usize;
            self.col = next_col as usize;
        } else {
            // Bounce
            // If the next col is a wall or outside the bounds of the box
            if next_col <= 0 || next_col >= width as isize - 1 {
                //flip from - to + movement direction on the x axis
                self.col_step *= -1;
            }

            // If the next row is a wall or outside the bounds of the box
            if next_row <= 0 || next_row >= height as isize - 1 {
                // flip from from - to + on the y axis
                self.row_step *= -1;
            }
        }
    }
}

struct Grid {
    width: usize,
    height: usize,
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

            // this loop is a little weird but it allows us to do an arbirtary number of balls. Maybe it can be made better
            let mut row = 1;
            let mut col = 1;
            for idx in 0..num_balls {
                balls.push(Ball::new(idx / width, idx % width, 1, 1));
                row = row + 1 % height;
                if row == height - 1 {
                    col = col + 1 % width;
                }
            }

            Ok(Self {
                balls,
                width,
                height,
                ball,
                wall,
                buf_writer: io::BufWriter::new(io::stdout()),
                terminated: false,
                wait_time: 33,
                run_time,
            })
        } else {
            Err("ERROR: Row and Col must each be larger then 2")
        }
    }

    // Maybe this can be removed but mostly its here beause the buf_writer isnt ready in new
    fn initialize(&mut self) -> io::Result<()> {
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

    fn render(&mut self) -> io::Result<()> {
        write!(self.buf_writer, "\x1b[H")?;

        for row in 0..self.height {
            for col in 0..self.width {
                let ball_in_cell = self
                    .balls
                    .iter()
                    .any(|ball| ball.row == row && ball.col == col);

                if !(row > 0 && row < self.height - 1 && col > 0 && col < self.width - 1) {
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
            self.balls.iter_mut().for_each(|ball| {
                ball.move_ball(self.width, self.height);
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
    let mut grid = Grid::new(25, 13, '●', '█', 4, 30)?;
    grid.initialize()?;
    grid.run()?;
    grid.terminate()?;

    Ok(())
}
