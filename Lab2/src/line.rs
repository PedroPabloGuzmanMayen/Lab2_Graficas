
use crate::framebuffer::FrameBuffer;

pub trait Line {
    fn line(&mut self, x1: usize, y1: usize, x2: usize, y2:usize);
}

impl Line for FrameBuffer {
    fn line(&mut self, x1: usize, y1: usize, x2: usize, y2: usize) {
        let mut x = x1 as isize;
        let mut y = y1 as isize;
        let dx = (x2 as isize - x1 as isize).abs();
        let dy = (y2 as isize - y1 as isize).abs();
        let sx = if x1 < x2 { 1 } else { -1 };
        let sy = if y1 < y2 { 1 } else { -1 };
        let mut err = if dx > dy { dx / 2 } else { -dy / 2 };

        loop {
            self.point(x as usize, y as usize);
            if x == x2 as isize && y == y2 as isize { break; }
            let e2 = err;
            if e2 > -dx {
                err -= dy;
                x += sx;
            }
            if e2 < dy {
                err += dx;
                y += sy;
            }
        }
    }
}
