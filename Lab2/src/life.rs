use crate::framebuffer::FrameBuffer;  

pub trait Life {

    fn life(&mut self);

}

impl Life for FrameBuffer{
    fn life(&mut self){
        let mut live_cells_counter: usize = 0;
        let mut neighbours: Vec<u32> = Vec::new();
        for i in 0..self.buffer.len(){
            neighbours = get_adyacent_values(&self.cast_buffer(), i, self.width);
            live_cells_counter = neighbours.iter().filter(|&&x| x == self.current_color.to_hex()).count();
            if self.buffer[i].to_hex() == self.current_color.to_hex() {
                if live_cells_counter < 2{
                    self.buffer[i] = self.background_color;
                }
                else if live_cells_counter > 3 {
                    self.buffer[i] = self.background_color;
                }
            }
            else {
                if live_cells_counter == 3 {
                    self.buffer[i] = self.current_color;
                }
            }
        }
        neighbours.clear();

    }
}

fn get_adyacent_values(vec: &Vec<u32>, index: usize, width:usize) -> Vec<u32>{
    let mut values:Vec<u32> = Vec::new();
    //Primera casilla
    if index == 0 {
        values.extend(vec![vec[index+1], vec[index+width], vec[index+width+1]]);
    }
    //Ente la primera y última casilla del primer renglón
    else if index > 0 && index < width-1{
        values.extend(vec![vec[index-1], vec[index+1], vec[index+width], vec[index+width+1], vec[index+width-1]]);
    }
    //última casilla del primer renglón
    else if index == width-1{
        values.extend(vec![vec[index-1], vec[index+width-1], vec[index + width]]);
    }
    //Primera casilla de cada englón
    else if index%width == 0 && index != width*(width-1){
        values.extend(vec![vec[index-width], vec[index-width+1], vec[index+1], vec[index+width], vec[index+width+1]]);
    }
    //Primer casilla del último renglón
    else if index == width*(width-1){
        values.extend(vec![vec[index-width], vec[index-width+1], vec[index+1]]);
    }
    //Última casilla de cada renglón
    else if (index+1)%width == 0 && index != (width*width) -1{
        values.extend(vec![vec[index-width], vec[index-width-1], vec[index-1], vec[index+width-1], vec[index+width]]);
    }
    //Última casilla del último renglón
    else if index == (width*width)-1{
        values.extend(vec![vec[index-width], vec[index-1], vec[index-width-1]]);
    }
    //Entre la primera y última casilla del primer renglón
    else if index > width*(width-1) && index < (width*width)-1 {
        values.extend(vec![vec[index-1], vec[index+1], vec[index-width], vec[index-width-1], vec[index-width+1]]);
    }
    //Cualquier casilla que no cumpla esas condiciones
    else{
        values.extend(vec![vec[index-1], vec[index+1], vec[index-width-1], vec[index-width], vec[index-width+1], vec[index+width-1], vec[index+width], vec[index+width+1]]);
    }

    values

}