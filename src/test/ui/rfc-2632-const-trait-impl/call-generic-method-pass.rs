//! Basic test for calling methods on generic type parameters in `const fn`.

// check-pass

#![feature(const_trait_impl)]
#![feature(const_fn_trait_bound)]
#![allow(incomplete_features)]

struct S;

impl const PartialEq for S {
    fn eq(&self, _: &S) -> bool {
        true
    }
}

const fn equals_self<T: PartialEq>(t: &T) -> bool {
    *t == *t
}

pub const EQ: bool = equals_self(&S);

fn main() {}
