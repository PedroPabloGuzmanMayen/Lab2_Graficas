use crate::line::Line;
use crate::framebuffer::FrameBuffer;  

pub trait Polygon {
    fn polygon(&mut self, vec: Vec<[usize; 2]>);
    fn fill_polygon(&mut self, vec:Vec<[usize;2]>);
}

impl Polygon for FrameBuffer {
    fn polygon(&mut self, vec: Vec<[usize; 2]>) {
        // Draw the polygon edges
        for i in 0..vec.len() {
            let (x1, y1) = (vec[i][0], vec[i][1]);
            let (x2, y2) = if i == vec.len() - 1 { (vec[0][0], vec[0][1]) } else { (vec[i + 1][0], vec[i + 1][1]) };
            self.line(x1, y1, x2, y2);
        }
    }

    fn fill_polygon(&mut self, vec: Vec<[usize; 2]>) {
        let (max_x, min_x, max_y, min_y) = get_max_limits(&vec);
        let mut intersections:Vec<usize> = Vec::new();
        for y in (min_y+1)..=(max_y-1) {

            for x in min_x..=max_x {
                if self.get_color(x, y).to_hex() != self.background_color.to_hex(){
                    intersections.push(self.width*y+x);
                }
                
            }

            if intersections.len() > 1 {

                let delete_points = delete_adjacent_points(&intersections);

                for n in 0..delete_points.len(){
                    intersections.remove(delete_points[n]);
                }

            
                if intersections.len()%2 != 0 {
                    intersections.push(intersections[intersections.len()-1]);
                    let idx = intersections.len()-2;
                    intersections[idx] = intersections[intersections.len()-1]
                }

                for k in (0..intersections.len()-1).step_by(2){
                    let start = intersections[k];
                    let end = intersections[k + 1];
                    for l in start..=end {
                        self.buffer[l] = self.current_color;
                    }
                }
            }
            intersections.clear();
        }
    }
}



pub fn get_max_limits( vec: &Vec<[usize; 2]>) ->(usize, usize, usize, usize) {
    let x_point_storage: Vec<usize> = vec.iter().map(|arr| arr[0]).collect();
    let y_point_storage: Vec<usize> =vec.iter().map(|arr| arr[1]).collect();

    (*x_point_storage.iter().max().unwrap(),*x_point_storage.iter().min().unwrap(),
    *y_point_storage.iter().max().unwrap()
    ,*y_point_storage.iter().min().unwrap())
}

pub fn delete_adjacent_points(vec: &Vec<usize>) -> Vec<usize> {
    let mut points_to_delete: Vec<usize> = Vec::new();
    let len = vec.len();
    for i in 0..len {
        if i + 1 < len && vec[i + 1] == vec[i] + 1 {
            points_to_delete.push(i);
        } else if i + 2 < len && vec[i + 2] == vec[i] + 2 {
            points_to_delete.push(i);
        }
    }
    points_to_delete
}



