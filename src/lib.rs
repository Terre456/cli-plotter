use std::io::{Stdout, Write};

use ansi_rgb::{Background, Foreground};
use crossterm::{cursor::MoveTo, queue};
use lazy_static::lazy_static;
use num_traits::ToPrimitive;

use rgb::{RGB8, Rgb};
use terminal_colorsaurus::{QueryOptions, background_color};
#[derive(Debug)]
pub enum Errors {
  EmptyData,
  UncastableValue,
}
pub struct Bar {
  // remove pubs
  pub pos: (u16, u16),
  pub size: f32,
  pub color: RGB8,
}
pub struct Plot {
  data: Vec<f32>,
  size: u16,
  fill_ratio: f32,
  color: Colors,
}
pub enum Colors {
  Rgb(RGB8),
  Rainbow,
  Palette(Vec<RGB8>),
}
lazy_static! {
  static ref RAINBOW_COLORS: Vec<RGB8> = vec![
    Rgb {
      r: 243,
      g: 128,
      b: 31
    },
    Rgb {
      r: 190,
      g: 202,
      b: 1
    },
    Rgb {
      r: 113,
      g: 243,
      b: 19
    },
    Rgb {
      r: 42,
      g: 248,
      b: 78
    },
    Rgb {
      r: 3,
      g: 202,
      b: 156
    },
    Rgb {
      r: 12,
      g: 128,
      b: 224
    },
    Rgb {
      r: 65,
      g: 53,
      b: 254
    },
    Rgb {
      r: 142,
      g: 7,
      b: 236
    },
    Rgb {
      r: 213,
      g: 7,
      b: 177
    },
    Rgb {
      r: 243,
      g: 128,
      b: 31
    },
    Rgb {
      r: 252,
      g: 53,
      b: 99
    },
  ];
}
impl Bar {
  pub fn show(self, stdout: &mut Stdout) {
    let sign = self.size.signum() as i32;
    let bar_size = (self.size * 8.0).round() as i32;
    let block_count = bar_size / 8;
    let spec_count = bar_size % 8;
    if sign < 0 {
      self.show_negative(block_count, spec_count, stdout);
    } else {
      self.show_positive(block_count, spec_count, stdout);
    }
    stdout.flush().unwrap();
  }
  fn show_positive(self, block_count: i32, spec_count: i32, stdout: &mut Stdout) {
    let char = char::from_u32(0x2580_i32.saturating_add(spec_count) as u32)
      .unwrap()
      .to_string()
      .repeat(2)
      .fg(self.color);
    let block = char::from_u32(0x2588).unwrap().to_string().repeat(2);
    for i in 0..block_count {
      queue!(
        stdout,
        MoveTo(self.pos.0, self.pos.1.saturating_sub(i as u16))
      )
      .unwrap();
      print!("{block}");
    }
    if spec_count != 0 {
      queue!(
        stdout,
        MoveTo(self.pos.0, (self.pos.1 as i32 - block_count) as u16)
      )
      .unwrap();
      print!("{char}");
    }
  }
  fn show_negative(self, block_count: i32, spec_count: i32, stdout: &mut Stdout) {
    let (bg_r, bg_g, bg_b) = match background_color(QueryOptions::default()).ok() {
      Some(v) => v.scale_to_8bit(),
      None => Plot::DEFAULT_BG_COLOR,
    };
    let bg_color = rgb::RGB8::new(bg_r, bg_g, bg_b);

    let char = char::from_u32(0x2588_i32.saturating_add(spec_count) as u32)
      .unwrap()
      .to_string()
      .repeat(2)
      .fg(bg_color)
      .bg(self.color); // inverting background and foreground colors to invert the characters (ie negative)
    let block = char::from_u32(0x2588).unwrap().to_string().repeat(2);

    for i in 1..-block_count + 1 {
      queue!(stdout, MoveTo(self.pos.0, self.pos.1 + i as u16)).unwrap();
      print!("{block}"); }
    if spec_count != 0 {
      queue!(
        stdout,
        MoveTo(self.pos.0, (self.pos.1 as i32 - block_count + 1) as u16)
      )
      .unwrap();
      print!("{char}");
    }
  }
}

impl Plot {
  const DEFAULT_BG_COLOR : (u8, u8, u8) = (24, 24, 24);
  pub fn new<T: ToPrimitive>(data: Vec<T>, size: u16) -> Plot {
    let default_color = Colors::Rgb(RGB8::new(200, 200, 200));
    let data: Vec<f32> = data.into_iter().map(|v| v.to_f32().unwrap()).collect();
    Plot {
      data,
      size,
      fill_ratio: 1.0,
      color: default_color,
    }
  }
  pub fn set_fill_ratio(&mut self, v: f32) {
    self.fill_ratio = f32::max(1.0, v);
  }
  pub fn set_color(&mut self, color: Colors) {
    self.color = color;
  }

  pub fn set_size(&mut self, size: u16) {
    self.size = size;
  }
  pub fn show(self, pos: (u16, u16), stdout: &mut Stdout) {
    let max = self.get_max();
    let bar_array = self.get_bar_array(pos, max);
    bar_array.into_iter().for_each(|bar| {
      bar.show(stdout);
    });
  }
  fn get_max(&self) -> f32 {
    let v = match self.data.get(0) {
      Some(v) => v,
      None => return 0.0,
    };
    self.data.iter().fold(v.to_f32().unwrap(), |a, x| {
      match x.to_f32().unwrap().total_cmp(&a) {
        std::cmp::Ordering::Greater => x.to_f32().unwrap(),
        _ => a,
      }
    })
  }
  fn get_bar_array(&self, pos: (u16, u16), max_value: f32) -> Vec<Bar> {
    let color_vec = self.get_color_vec();
    let mut colors_iter = color_vec.iter().cycle();
    let mut x = 0;
    self
      .data
      .iter()
      .map(|v| {
        let size = v / (max_value * self.fill_ratio) * self.size as f32;
        Bar {
          pos: {
            let px = pos.0 + x;
            x += 2;
            (px, pos.1)
          },
          size,
          color: *colors_iter.next().unwrap(),
        }
      })
      .collect()
  }
  fn get_color_vec(&self) -> Vec<Rgb<u8>>{
    match &self.color {
      Colors::Rgb(rgb) => vec![rgb.clone()],
      Colors::Rainbow => RAINBOW_COLORS.to_vec(),
      Colors::Palette(rgbs) => rgbs.clone(),
    }
  }
}
