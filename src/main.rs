use cli_plotter::{Colors, Plot};
use crossterm::{cursor::MoveTo, execute, terminal::Clear};
use rand::{RngExt, rng};
use std::io::stdout;

fn main() {
  sample_test();
}

fn sample_test() {
  let values = random_sample(-3.0, 6.0, 10);
  let mut stdout = stdout();
  let mut plot = Plot::new(values, 7);
  plot.set_color(Colors::Rainbow);
  execute!(stdout, Clear(crossterm::terminal::ClearType::All)).unwrap();
  plot.show((3, 9), &mut stdout);
  execute!(stdout, MoveTo(0, 0)).unwrap();
}
fn random_sample(start: f32, stop: f32, count: u32) -> Vec<f32> {
  let mut v = vec![];
  let mut rng = rng();
  for _ in 0..count {
    v.push(rng.random_range(start..stop));
  }
  v
}
