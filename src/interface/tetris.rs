use crate::engine::position::Pos;
use crate::engine::shape::Shape;
use std::{collections::HashSet, mem};

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub enum Direction {
    Left,
    Right,
}

#[derive(Debug)]
pub struct Tetris {
    width: i32,
    height: i32,
    curr_shape: Shape,
    fixed_shapes: Vec<Shape>,
    game_over: bool,
    score: u32,
}

impl Tetris {
    pub fn new(width: u32, height: u32) -> Self {
        Self {
            width: width as i32,
            height: height as i32,
            curr_shape: &Shape::new_rand() + Pos((width as i32) / 2, 0),
            fixed_shapes: vec![],
            game_over: false,
            score: 0,
        }
    }

    pub fn reset(&mut self) {
        self.curr_shape = &Shape::new_rand() + Pos(self.width / 2, 0);
        self.fixed_shapes.clear();
        self.game_over = false;
        self.score = 0;
    }

    pub fn is_game_over(&self) -> bool {
        self.game_over
    }

    pub fn score(&self) -> u32 {
        self.score
    }

    pub fn width(&self) -> i32 {
        self.width
    }

    pub fn height(&self) -> i32 {
        self.height
    }

    pub fn is_out_of_bounds(&self, shape: &Shape) -> bool {
        !shape
            .iter_positions()
            .all(|pos| 0 <= pos.0 && pos.0 < self.width && 0 <= pos.1 && pos.1 < self.height)
    }

    pub fn is_colliding(&self, shape: &Shape) -> bool {
        self.fixed_shapes
            .iter()
            .any(|fixed_shape| fixed_shape.collides_with(shape))
    }

    pub fn iter_position(&self) -> impl Iterator<Item = Pos> {
        let height = self.height;
        let width = self.width;
        (0..height).flat_map(move |y| (0..width).map(move |x| Pos(x, y)))
    }

    pub fn get(&self, pos: Pos) -> Option<&'static str> {
        if self.curr_shape.has_position(pos) {
            Some(self.curr_shape.typ())
        } else {
            self.fixed_shapes
                .iter()
                .find(|shape| shape.has_position(pos))
                .map(|shape| shape.typ())
        }
    }

    pub fn tick(&mut self) {
        if self.game_over {
            return;
        }

        let translated_curr_shape = &self.curr_shape + Pos(0, 1);

        if self.is_out_of_bounds(&translated_curr_shape)
            || self.is_colliding(&translated_curr_shape)
        {
            let new_fixed_shape = mem::replace(
                &mut self.curr_shape,
                &Shape::new_rand() + Pos(self.width / 2, 0),
            );

            self.fixed_shapes.push(new_fixed_shape);
            self.remove_full_lines();

            if self.is_colliding(&self.curr_shape) {
                self.game_over = true;
            }
        } else {
            self.curr_shape = translated_curr_shape;
        }
    }

    pub fn shift(&mut self, direction: Direction) {
        if self.game_over {
            return;
        }

        let translated_curr_shape = &self.curr_shape
            + match direction {
                Direction::Left => Pos(-1, 0),
                Direction::Right => Pos(1, 0),
            };

        if !self.is_out_of_bounds(&translated_curr_shape)
            && !self.is_colliding(&translated_curr_shape)
        {
            self.curr_shape = translated_curr_shape;
        }
    }

    pub fn rotate(&mut self) {
        if self.game_over {
            return;
        }

        let rotated_curr_shape = self.curr_shape.rotated();

        if !self.is_out_of_bounds(&rotated_curr_shape) && !self.is_colliding(&rotated_curr_shape) {
            self.curr_shape = rotated_curr_shape;
        }
    }

    pub fn drop(&mut self) {
        if self.game_over {
            return;
        }

        while !self.is_out_of_bounds(&(&self.curr_shape + Pos(0, 1)))
            && !self.is_colliding(&(&self.curr_shape + Pos(0, 1)))
        {
            self.curr_shape = &self.curr_shape + Pos(0, 1);
        }
        self.tick(); // Finaliza a peça
    }

    pub fn is_line_full(&self, y: i32) -> bool {
        self.fixed_shapes
            .iter()
            .flat_map(|shape| shape.iter_positions())
            .filter(|pos| pos.1 == y)
            .collect::<HashSet<_>>()
            .len()
            == self.width as usize
    }

    fn remove_line(&mut self, y: i32) {
        for shape in self.fixed_shapes.iter_mut() {
            shape.remove_line(y)
        }
    }

    fn remove_full_lines(&mut self) {
        let mut lines_removed = 0;
        for line in 0..self.height {
            if self.is_line_full(line) {
                self.remove_line(line);
                lines_removed += 1;
            }
        }
        // Pontuação: 100 por linha, com bônus para múltiplas linhas
        self.score += match lines_removed {
            1 => 100,
            2 => 300,
            3 => 500,
            4 => 800,
            _ => 0,
        };
    }
}

#[cfg(test)]
mod tests {
    use super::Tetris;

    #[test]
    fn tests() {
        let mut tetris = Tetris::new(10, 30);
        tetris.tick();
        tetris.tick();
        tetris.tick();
        println!("{:#?}", tetris);
    }
}
