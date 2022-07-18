pub mod factorial {
    use std::ops::{Div, Mul};

    pub struct Factorial<T> {
        fac: Vec<T>,
    }

    impl<T> Default for Factorial<T>
    where
        T: Clone + From<usize> + Mul<Output = T> + Div<Output = T>,
    {
        fn default() -> Self {
            Self::new()
        }
    }

    impl<T> Factorial<T>
    where
        T: Clone + From<usize> + Mul<Output = T> + Div<Output = T>,
    {
        /// Constructs a new, empty `Factorial<T>`.
        pub fn new() -> Self {
            Self {
                fac: vec![T::from(1)],
            }
        }

        /// Returns the factorial of `n`.
        pub fn factorial(&mut self, n: usize) -> T {
            if self.fac.len() < n + 1 {
                for i in (self.fac.len() - 1)..n {
                    self.fac.push(self.fac[i].clone() * (i + 1).into());
                }
            }
            self.fac[n].clone()
        }

        /// Returns the number of choices when selecting `k` from n and arranging them in a row.
        pub fn permutation(&mut self, n: usize, k: usize) -> T {
            if n < k {
                T::from(0)
            } else {
                self.factorial(n) / self.factorial(n - k)
            }
        }

        /// Returns the number of choices to select `k` from `n`.
        pub fn combination(&mut self, n: usize, k: usize) -> T {
            if n < k {
                T::from(0)
            } else {
                self.factorial(n) / (self.factorial(k) * self.factorial(n - k))
            }
        }
    }
}
