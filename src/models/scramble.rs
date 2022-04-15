
#[derive(PartialEq, Clone, Copy)]
enum Face {
    Up,
    Right,
    Front,
    Down,
    Left,
    Back,
}

use std::fmt::{Formatter, Display, Result};

use Face::*;

impl Face {
    fn is_opposite(&self, other: Face) -> bool {
        (*self as u8).abs_diff(other as u8) == 3
    }
}

impl Distribution<Face> for Standard {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> Face {
        let index: u8 = rng.gen_range(0..6);
        match index {
            0 => Up,
            1 => Right,
            2 => Front,
            3 => Down,
            4 => Left,
            5 => Back,
            _ => unreachable!(),
        }
    }
}

impl Display for Face {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(f, "{}", match self {
            Up => "U",
            Right => "R",
            Front => "F",
            Down => "D",
            Left => "L",
            Back => "B",
        })
    }
}

#[derive(PartialEq, Clone, Copy)]
enum Turn {
    Clockwise,
    Half,
    CounterClockwise,
}

use Turn::*;
use rand::prelude::*;
use rand::distributions::Standard;

impl Distribution<Turn> for Standard {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> Turn {
        let index: u8 = rng.gen_range(0..3);
        match index {
            0 => Clockwise,
            1 => Half,
            2 => CounterClockwise,
            _ => unreachable!(),
        }
    }
}

impl Display for Turn {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(f, "{}", match self {
            Clockwise => "",
            Half => "2",
            CounterClockwise => "'",
        })
    }
}

#[derive(PartialEq, Clone, Copy)]
struct Move(Face, Turn);

impl Distribution<Move> for Standard {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> Move {
        Move(rng.gen::<Face>(), rng.gen::<Turn>())
    }
}

impl Display for Move {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(f, "{}{}", self.0, self.1)
    }
}

#[derive(PartialEq, Clone)]
pub struct Scramble {
    moves: Vec<Move>,
}

impl Distribution<Scramble> for Standard {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> Scramble {
        let mut moves: Vec<Move> = vec![];
        
        while moves.len() < 20 {
            let m = rng.gen::<Move>();
            if !moves.is_empty() && moves[moves.len() - 1].0 == m.0 {
                continue;
            }
            if moves.len() > 1 && moves[moves.len() - 1].0.is_opposite(m.0) && moves[moves.len() - 2].0 == m.0 {
                continue;
            }

            moves.push(m);
        }

        Scramble { moves }
    }
}

impl Display for Scramble {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(f, "{}", self.moves.iter().map(|m| format!("{m}")).collect::<Vec<String>>().join(" "))
    }
}
