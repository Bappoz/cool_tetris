use std::mem;
use crate::engine::shape::{Pos, Shape};


#[derive(Debug)]
pub struct Tetris {
    width: i32,
    height: i32,
    curr_shape: Shape,
    fixed_shapes: Vec<Shape>
}


impl Tetris {
    pub fn new(width: u32, height: u32) -> Self {
        Self {
            width: width as i32,
            height: height as i32,
            curr_shape: &Shape::new_rand() + Pos((width as i32) / 2, 0),
            fixed_shapes: vec![],
        }
    }

    pub fn is_out_of_bounds(&self, shape: &Shape) -> bool {
        shape.positions().all(|pos| 0 <= pos.0 && pos.0 < self.width && 0 <= pos.1 && pos.1 < self.height)
    }

    pub fn is_colliding(&self, shape: &Shape) -> bool {
        self.fixed_shapes.iter().any(|fixed_shape| fixed_shape.collides_with(shape))
    }

    pub fn tick(&mut self){
        let translated_curr_shape = &self.curr_shape + Pos(0,1);
        if self.is_out_of_bounds(&translated_curr_shape)
            || self.is_colliding(&translated_curr_shape) {

            let new_fixed_shape = mem::replace(
                &mut self.curr_shape,
                &Shape::new_rand() + Pos(self.width / 2, 0),
            );
            self.fixed_shapes.push(new_fixed_shape);
        } else {
            self.curr_shape = translated_curr_shape;
        }
    }
}



#[cfg(test)]
mod tests{
    use super::Tetris;

    #[test]
    fn test() {
        println!("{:#?}",Tetris::new(5, 10))
    }
}