mod framebuffer;
mod line;
mod color;
mod bmp;
mod polygon;
mod patterns;
mod life;
use crate::framebuffer::FrameBuffer;
use crate::line::Line;
use crate::color::Color;
use crate::polygon::{Polygon, get_max_limits};
use crate::life::Life;
use minifb::{Window, WindowOptions, Key};
use std::time::Duration;
fn main() {
  let window_width = 1000;
  let window_height = 1000;

  let framebuffer_width = 100;
  let framebuffer_height = 100;

  let frame_delay = Duration::from_millis(16);

  let mut framebuffer = framebuffer::FrameBuffer::new(framebuffer_width, framebuffer_height);
  framebuffer.set_current_color(Color::new(255,0,0));
  framebuffer.randomize_patterns();

  let mut window = Window::new(
      "Rust Graphics - Render Loop Example",
      window_width,
      window_height,
      WindowOptions::default(),
  ).unwrap();

  let mut x = 1 as i32;
  let mut y = 1 as i32;
  let mut speed = 1 as i32;
  let mut y_speed = 1 as i32;

  while window.is_open() {
      // Listen to inputs
      if window.is_key_down(Key::Escape) {
          break;
      }

      // Takes a screenshot 
      if window.is_key_down(Key::Enter) {
          framebuffer.write_to_bmp("screenshot.bmp").unwrap();
      }

      framebuffer.life();

      // Update the window with the framebuffer contents
      window
          .update_with_buffer(&framebuffer.cast_buffer(), framebuffer_width, framebuffer_height)
          .unwrap();

      std::thread::sleep(frame_delay);
  }
  
  framebuffer.write_to_bmp("out.bmp");
}
