use crossterm::{cursor, terminal};
use rand::Rng;
use std::{f64::consts::PI, io::Write};

fn main() -> anyhow::Result<()> {
    let c = Circle { radius: 1.0 };

    let mut total_tally = 0;
    let mut inside_tally = 0;

    let mut stdout = std::io::stdout();
    crossterm::execute!(stdout, terminal::Clear(terminal::ClearType::All))?;

    loop {
        let mut rng = rand::thread_rng();

        let p = Point {
            x: rng.gen_range(-c.radius, c.radius),
            y: rng.gen_range(-c.radius, c.radius),
        };

        if p.is_inside(c) {
            inside_tally += 1
        }

        total_tally += 1;

        let approximation = 4.0 * (inside_tally as f64 / total_tally as f64);
        let percent_diff = (PI - approximation).abs() / ((PI + approximation) / 2.0);

        crossterm::execute!(stdout, cursor::MoveTo(0, 0))?;
        print!(
            "π ≈ {:.10} ({:.10}% difference)",
            approximation, percent_diff
        );
    }
}

struct Point {
    x: f64,
    y: f64,
}

impl Point {
    fn is_inside(&self, c: Circle) -> bool {
        let dist_to_centre = self.x.powi(2) + self.y.powi(2);
        let dist_to_centre = dist_to_centre.sqrt();

        dist_to_centre < c.radius
    }
}

#[derive(Clone, Copy)]
struct Circle {
    radius: f64,
}
