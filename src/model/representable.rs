/// isomorphism between `Representable<A>` and `Representable<A>::Index -> A`
pub trait Representable<A> {
    type Index;

    fn tabulate<F: Fn(Self::Index) -> A>(f: F) -> Self;

    fn index(self, index: Self::Index) -> A;
}

impl<A> Representable<A> for (A, A) {
    type Index = bool;

    fn tabulate<F: Fn(Self::Index) -> A>(f: F) -> Self {
        (f(false), f(true))
    }

    fn index(self, index: Self::Index) -> A {
        if index {
            self.1
        } else {
            self.0
        }
    }
}

// impl<A: Representable, B: Representable> Representable<A> for  {
//     type Index = (A::Index, B::Index);
//
//     fn tabulate<F: Fn(Self::Index) -> A>(f: F) -> Self {
//         todo!()
//     }
//
//     fn index(&self, index: Self::Index) -> A {
//         todo!()
//     }
// }
