use rand::prelude::*;

use crate::models::scramble::Scramble;

pub fn generate_scramble() -> String {
    format!("{}", random::<Scramble>())
}

#[cfg(test)]
mod test {
    #[test]
    fn test() {
        println!("{}", super::generate_scramble());
    }
}
