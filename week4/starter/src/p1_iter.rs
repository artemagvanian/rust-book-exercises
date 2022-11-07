//! P1: Cartesian product iterator
//!
//! To get experience with traits and generics, you will implement a new kind
//! of iterator: cartesian product. The product of two iterators is the set
//! of all pairs of items from each iterator. For example:
//!
//! [1, 2] x [3, 4]  =  [(1, 3), (1, 4), (2, 3), (2, 4)]
//!
//! Your task is to design all the structs, traits, and impls that are needed
//! to make this code snippet possible:
//!
//! ```ignore
//! let h1 = hashset![1, 2];
//! let h2 = hashset![3, 4];
//! let product =
//!   h1.into_iter()
//!   .cartesian_product(h2.into_iter())
//!   .collect::<HashSet<_>>();
//! ```
//!
//! That is, there should be a method `cartesian_product` which can be called
//! on *any* iterator, such as the one produced by `HashSet::into_iter`. This method
//! returns a structure that implements the `Iterator` trait, allowing one to call
//! methods like `collect`.
//!
//! The snippet above is provided as a unit test, which you can run with
//! `cargo test product`. The test will not compile until you build the API.
//!
//! To get you started, I would read Rust's documentation on how to implement an iterator:
//! https://doc.rust-lang.org/std/iter/index.html#implementing-iterator


// Your implementation goes here!
struct CacheableIterator<T, A>
    where T: Iterator<Item=A>, A: Clone {
    iter: T,
    cache: Vec<A>,
    last_idx: Option<usize>
}

impl<T, A> CacheableIterator<T, A>
    where T: Iterator<Item=A>, A: Clone {
    fn new(iter: T) -> CacheableIterator<T, A> {
        CacheableIterator { iter, cache: vec![], last_idx: None }
    }

    fn next(&mut self) -> Option<A> {
        let next_elt = self.iter.next();
        match next_elt {
            None => None,
            Some(next_val) => {
                self.cache.push(next_val.clone());
                match self.last_idx {
                    None => { self.last_idx = Some(0) }
                    Some(prev_last_idx) => { self.last_idx = Some(prev_last_idx + 1) }
                }
                return Some(next_val);
            }
        }
    }

    fn ith(&mut self, i: usize) -> Option<A> {
        while self.last_idx.and_then(|idx| Some(idx < i)).unwrap_or(true) {
            match self.next() {
                None => { return None; },
                Some(_) => ()
            }
        }
        return Some(self.cache[i].clone());
    }
}

struct CartesianIterator<T, U, A, B>
    where T: Iterator<Item=A>, U: Iterator<Item=B>, A: Clone, B: Clone {
    first: CacheableIterator<T, A>,
    second: CacheableIterator<U, B>,
    idx_first: usize,
    idx_second: usize,
}

impl<T, U, A, B> CartesianIterator<T, U, A, B>
    where T: Iterator<Item=A>, U: Iterator<Item=B>, A: Clone, B: Clone {
    fn new(first: T, second: U) -> CartesianIterator<T, U, A, B> {
        CartesianIterator { first: CacheableIterator::new(first),
            second: CacheableIterator::new(second), idx_first: 0, idx_second: 0}
    }
}

impl<T, U, A, B> Iterator for CartesianIterator<T, U, A, B>
    where T: Iterator<Item=A>, U: Iterator<Item=B>, A: Clone, B: Clone {
    type Item = (A, B);

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            let nxt_a = self.first.ith(self.idx_first);
            let nxt_b = self.second.ith(self.idx_second);
            match nxt_a {
                None => { return None; },
                Some(a) => {
                    match nxt_b {
                        None => {
                            self.idx_first += 1;
                            self.idx_second = 0;
                            continue;
                        }
                        Some(b) => {
                            self.idx_second += 1;
                            return Some((a, b));
                        }
                    }
                }
            }
        }
    }
}

trait Cartesian<T, U, A, B>
    where T: Iterator<Item=A>, U: Iterator<Item=B>, A: Clone, B: Clone {
    fn cartesian_product(self, other: U) -> CartesianIterator<T, U, A, B>;
}

impl<T, U, A, B> Cartesian<T, U, A, B> for T
    where T: Iterator<Item=A>, U: Iterator<Item=B>, A: Clone, B: Clone {
    fn cartesian_product(self, other: U) -> CartesianIterator<T, U, A, B> {
        CartesianIterator::new(self, other)
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use maplit::hashset;
    use std::collections::HashSet;

    #[test]
    fn cartesian_product_test() {
        let h1 = hashset![1, 2];
        let h2 = hashset![3, 4];
        let product = h1.into_iter().cartesian_product(h2.into_iter());
        assert_eq!(
            product.collect::<HashSet<_>>(),
            hashset![(1, 3), (1, 4), (2, 3), (2, 4)]
        );

        let v1 = vec![1, 2, 3];
        let v2 = vec![4, 5];
        let product = v1.into_iter().cartesian_product(v2.into_iter());
        assert_eq!(
            product.collect::<Vec<_>>(),
            vec![(1, 4), (1, 5), (2, 4), (2, 5), (3, 4), (3,5)]
        );
    }
}
