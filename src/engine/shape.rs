use crate::engine::position::Pos;
use std::collections::HashSet;
use std::ops::Add;

#[derive(Debug, Clone)]
pub struct Shape {
    typ: &'static str,
    positions: HashSet<Pos>,
    anchor: Pos,
}

macro_rules! impl_shape_constructor {
    ($( $new:ident $typ:literal: [ $( $pos:expr ),* ] anchored at $anchor:expr; )*) => {
        $(
            pub fn $new() -> Self {
                Self {
                    typ: $typ,
                    positions: [$( $pos ),*].into_iter().collect(),
                    anchor: $anchor,
                }
            }
        )*
    };
}

impl Shape {
    impl_shape_constructor! {
        new_i "I": [Pos(0,0), Pos(1,0), Pos(2,0), Pos(3,0)] anchored at Pos(1, 0);
        new_o "O": [Pos(0,0), Pos(1,0), Pos(0,1), Pos(1,1)] anchored at Pos(0, 0);
        new_t "T": [Pos(0,0), Pos(1,0), Pos(2,0), Pos(1,1)] anchored at Pos(0, 0);
        new_j "J": [Pos(0,0), Pos(0,1), Pos(0,2), Pos(-1,2)] anchored at Pos(0, 1);
        new_l "L": [Pos(0,0), Pos(0,1), Pos(0,2), Pos(1,2)] anchored at Pos(0, 1);
        new_s "S": [Pos(0,0), Pos(1,0), Pos(0,1), Pos(-1,1)] anchored at Pos(0, 0);
        new_z "Z": [Pos(0,0), Pos(-1,0), Pos(0,1), Pos(-1,1)] anchored at Pos(0, 0);
    }

    pub fn new_rand() -> Self {
        let random = (rand::random::<f64>() * 7.0).floor() as u8;

        match random {
            0 => Self::new_i(),
            1 => Self::new_o(),
            2 => Self::new_t(),
            3 => Self::new_j(),
            4 => Self::new_l(),
            5 => Self::new_s(),
            6 => Self::new_z(),
            _ => unreachable!(),
        }
    }

    pub fn iter_positions(&self) -> impl Iterator<Item = Pos> + '_ {
        self.positions.iter().copied()
    }

    pub fn collides_with(&self, other: &Shape) -> bool {
        self.positions.intersection(&other.positions).count() > 0
    }

    pub fn typ(&self) -> &'static str {
        self.typ
    }

    pub fn rotated(&self) -> Self {
        let Pos(a, b) = self.anchor;
        Self {
            typ: self.typ, 
            positions: self
                .iter_positions()
                .map(|Pos(x, y)| Pos(-y + b + a, x - a + b))
                .collect(),
            anchor: self.anchor,
        }
    }

    pub fn remove_line(&mut self, y: i32) {
        self.positions = self
            .positions
            .iter()
            .copied()
            .filter(|pos| pos.1 != y)
            .map(|pos| {
                if pos.1 >= y {
                    pos
                } else {
                    Pos(pos.0, pos.1 + 1)
                }
            })
            .collect()
    }

    pub fn has_position(&self, pos: Pos) -> bool {
        self.positions.contains(&pos)
    }
}

impl Add<Pos> for &Shape {
    type Output = Shape;

    fn add(self, rhs: Pos) -> Self::Output {
        Shape {
            typ: self.typ,
            positions: self.positions.iter().map(|pos| *pos + rhs).collect(),
            anchor: self.anchor + rhs,
        }
    }
}
