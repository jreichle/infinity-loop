pub trait Semigroup {
    fn combine(&self, other: &Self) -> Self;
}

pub trait Monoid: Semigroup {
    const NEUTRAL: Self;
}
