/// isomorphism between `Representable<A>` and `Representable<A>::Index -> A`
pub trait Representable<A> {
    type Index;

    fn tabulate<F: Fn(Self::Index) -> A>(f: F) -> Self;

    fn index(&self, index: Self::Index) -> A;

    fn index_fn(&self) -> Box<dyn Fn(Self::Index) -> A + '_>
    where
        Self: Copy + Sized,
    {
        Box::new(move |i| Self::index(self, i))
    }
}

impl<A: Copy> Representable<A> for (A, A) {
    type Index = bool;

    fn tabulate<F: Fn(Self::Index) -> A>(f: F) -> Self {
        (f(false), f(true))
    }

    fn index(&self, index: Self::Index) -> A {
        if index {
            self.1
        } else {
            self.0
        }
    }
}
