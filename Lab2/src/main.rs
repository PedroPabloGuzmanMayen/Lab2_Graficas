mod framebuffer;
mod line;
mod color;
mod bmp;
mod polygon;
use crate::framebuffer::FrameBuffer;
use crate::line::Line;
use crate::color::Color;
use crate::polygon::{Polygon, get_max_limits};
use minifb::{Window, WindowOptions, Key};
use std::time::Duration;
fn main() {
  let window_width = 800;
  let window_height = 600;

  let framebuffer_width = 80;
  let framebuffer_height = 60;

  let frame_delay = Duration::from_millis(120);

  let mut framebuffer = framebuffer::FrameBuffer::new(framebuffer_width, framebuffer_height);

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

      // Prepare variables for rendering
      if x as usize >= framebuffer_width - 1 || x == 0 {
          speed = -speed;
      }
      if y as usize >= framebuffer_height - 1 || y == 0 {
          y_speed = -y_speed;
      }

      x += speed;
      y += y_speed;

      // Ensure x and y are within bounds
      let clamped_x = x.clamp(0, (framebuffer_width - 1) as i32) as usize;
      let clamped_y = y.clamp(0, (framebuffer_height - 1) as i32) as usize;

      // Clear the framebuffer
      framebuffer.set_background_color(Color::new(0, 0, 0));
      framebuffer.clear();

      // Draw some points
      framebuffer.set_current_color(Color::new(255, 255, 0));
      framebuffer.point(clamped_x, clamped_y);

      let casted_fb = framebuffer.cast_buffer();

      // Update the window with the framebuffer contents
      window
          .update_with_buffer(&casted_fb, framebuffer_width, framebuffer_height)
          .unwrap();

      std::thread::sleep(frame_delay);
  }
}
