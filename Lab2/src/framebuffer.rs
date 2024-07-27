use crate::color::Color;
use crate::bmp::write_bmp_file;
pub struct FrameBuffer {
    pub width: usize,
    pub height: usize,
    pub buffer: Vec<Color>,
    pub background_color: Color,
    pub current_color: Color
}

impl FrameBuffer {
    pub fn new (width: usize, height: usize) -> FrameBuffer {
        let default_color = Color::new(255,255,255);
        let buffer = vec![default_color; width*height];
        FrameBuffer {
            width,
            height,
            buffer,
            background_color: default_color,
            current_color: default_color
        }
    }

    pub fn clear(&mut self){
        self.buffer.fill(self.background_color);
    }

    pub fn point(&mut self, x:usize, y:usize){
        self.buffer[self.width * y + x] =  self.current_color;
    }

    pub fn set_background_color(&mut self, color:Color){
        self.background_color = color;
    }

    pub fn get_color(&mut self, x:usize, y:usize) -> Color {
        self.buffer[self.width * y + x]
    }

    pub fn set_current_color(&mut self, color:Color){
        self.current_color = color;
    }
    pub fn cast_buffer(&mut self) -> Vec<u32> {
        let mut casted_vector: Vec<u32> = Vec::with_capacity(self.buffer.len());  
        for i in 0..self.buffer.len() {
            casted_vector.push(self.buffer[i].to_hex());
        }
        casted_vector
    }
    pub fn write_to_bmp(&self, file_path: &str) -> std::io::Result<()> {
        let buffer: Vec<u32> = self.buffer.iter().map(|c| c.to_hex()).collect();
        write_bmp_file(file_path, &buffer, self.width, self.height)
    }
}
