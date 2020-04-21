use crossterm::{cursor, terminal};
use rand::Rng;
use std::{f64::consts::PI, io::Write};

fn main() -> anyhow::Result<()> {
    let mut total_tally = 0;
    let mut inside_tally = 0;

    let mut stdout = std::io::stdout();
    crossterm::execute!(stdout, terminal::Clear(terminal::ClearType::All))?;

    let mut best_approximation = 0.0;
    let mut lowest_difference = PI;

    loop {
        let mut rng = rand::thread_rng();

        let p = Point {
            x: rng.gen_range(-1.0, 1.0),
            y: rng.gen_range(-1.0, 1.0),
        };

        if p.in_unit_circle() {
            inside_tally += 1
        }

        total_tally += 1;

        let approximation = 4.0 * (inside_tally as f64 / total_tally as f64);
        let difference = (PI - approximation).abs();

        if difference < lowest_difference {
            best_approximation = approximation;
            lowest_difference = difference;
        }

        crossterm::execute!(stdout, cursor::MoveTo(0, 0))?;
        print!(
            "π ≈ {:.10} (difference = {:.10})",
            best_approximation, lowest_difference
        );
    }
}

struct Point {
    x: f64,
    y: f64,
}

impl Point {
    fn in_unit_circle(&self) -> bool {
        self.x.powi(2) + self.y.powi(2) < 1.0
    }
}
