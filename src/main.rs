use std::{ io::stdout};

use crossterm::{cursor::MoveTo, execute, terminal::Clear};

use cli_plotter::{Bar, Plot};
use rand::{RngExt, rng};
use rgb::RGB8;
use terminal_colorsaurus::{Color, QueryOptions, background_color};

fn main() {
  test1();
}
#[allow(unused)]
fn test2() {
  let values = random_sample(0.0, 6.0, 30);
  let mut stdout = stdout();
  let plot = Plot::new(values, 7);
  execute!(stdout, Clear(crossterm::terminal::ClearType::All)).unwrap();
  plot.show((3,9), &mut stdout);
  execute!(stdout, MoveTo(0, 0)).unwrap();
}
#[allow(unused)]

fn test1() {
  let b = Bar {
    pos: (5, 9),
    size: 2.3,
    color: RGB8::new(130, 230, 130)
  };
  let mut stdout = stdout();
  execute!(stdout, Clear(crossterm::terminal::ClearType::All)).unwrap();
  b.show(&mut stdout);
  execute!(stdout, MoveTo(0, 0)).unwrap();
}
#[allow(unused)]

fn test3 () {
  let bgcolor = background_color(QueryOptions::default()).unwrap_or(Color::rgb(24, 24, 24)).scale_to_8bit();
  println!("{:?}", bgcolor)
}
fn random_sample(start: f32, stop: f32 , count: u32) -> Vec<f32>{
  let mut v = vec![];
  let mut rng = rng();
  for _ in 0..count {
    v.push(rng.random_range(start..stop));
  };
  v
}
