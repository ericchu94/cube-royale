use std::fmt::{Formatter, Result, Display};

use rand::prelude::*;
use rand::distributions::Standard;

#[derive(PartialEq, Clone, Copy)]
enum Face {
    Up,
    Right,
    Front,
    Down,
    Left,
    Back,
}

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

enum Turn {
    Clockwise,
    Half,
    CounterClockwise,
}

use Turn::*;

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


fn random_move() -> (Face, Turn) {
    (random::<Face>(), random::<Turn>())
}

pub fn generate_scramble() -> String {
    let mut scramble: Vec<(Face, Turn)> = vec![];

    while scramble.len() < 20 {
        let m = random_move();
        if !scramble.is_empty() && scramble[scramble.len() - 1].0 == m.0 {
            continue;
        }
        if scramble.len() > 1 && scramble[scramble.len() - 1].0.is_opposite(m.0) && scramble[scramble.len() - 2].0 == m.0 {
            continue;
        }

        scramble.push(m);
    }

    scramble.into_iter().map(|(f, t)| format!("{f}{t}")).collect::<Vec<String>>().join(" ")
}

#[cfg(test)]
mod test {
    #[test]
    fn test() {
        println!("{}", super::generate_scramble());
    }
}
