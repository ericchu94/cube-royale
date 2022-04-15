use std::fmt::{Display, Formatter, Result};

use instant::Duration as InstantDuration;

#[derive(Default, PartialEq, PartialOrd, Clone, Copy)]
pub struct Duration(InstantDuration);

impl Duration {
    pub fn from_millis(millis: u64) -> Self {
        InstantDuration::from_millis(millis).into()
    }
}

impl Display for Duration {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(f, "{}.{:03}", self.0.as_secs(), self.0.subsec_millis())
    }
}

impl From<InstantDuration> for Duration {
    fn from(duration: InstantDuration) -> Self {
        Self(duration)
    }
}

#[cfg(test)]
mod test {
    #[test]
    fn test() {
        let d = super::Duration::from_millis(12345);
        assert_eq!("12.345".to_owned(), format!("{}", d));
    }
}
