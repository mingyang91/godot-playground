use std::fmt::{Display, Formatter};

#[derive(Debug)]
pub enum Moral {
    Good,
    Neutral,
    Evil,
}

impl Display for Moral {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Moral::Good => write!(f, "Good"),
            Moral::Neutral => write!(f, "Neutral"),
            Moral::Evil => write!(f, "Evil"),
        }
    }
}

#[derive(Debug)]
pub enum Ethical {
    Lawful,
    Neutral,
    Chaotic,
}

impl Display for Ethical {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Ethical::Lawful => write!(f, "Lawful"),
            Ethical::Neutral => write!(f, "Neutral"),
            Ethical::Chaotic => write!(f, "Chaotic"),
        }
    }
}

#[derive(Debug)]
pub struct Alignment {
    moral: Moral,
    ethical: Ethical,
}

impl Display for Alignment {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?} {:?}", self.moral, self.ethical)
    }
}

impl Alignment {
    pub fn new(moral: Moral, ethical: Ethical) -> Self {
        Alignment { moral, ethical }
    }
}