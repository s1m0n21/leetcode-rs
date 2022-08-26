mod solutions;
pub mod utils;

fn main() {}

#[cfg(test)]
mod tests {
    use std::cmp::PartialEq;
    use std::fmt::Debug;
    use std::time::Instant;

    use super::*;

    struct TestCase<I, E>
        where
            I: Debug,
            E: PartialEq + Debug,
    {
        func: fn(I) -> E,
    }

    impl<I, E> TestCase<I, E>
        where
            I: Debug,
            E: PartialEq + Debug,
    {
        fn new(f: fn(I) -> E) -> TestCase<I, E> {
            return TestCase { func: f };
        }

        fn run_test(&self, i: I, e: E) {
            print!("IN: {:?} | ", &i);

            let st = Instant::now();
            let out = (self.func)(i);

            if out == e {
                print!("ELAPSED: {:?} | ", st.elapsed());
                print!("\x1b[32mPASS\x1b[0m\n");
            } else {
                print!("EXP: \x1b[32m{:?}\x1b[0m | ", e);
                print!("OUT: \x1b[31m{:?}\x1b[0m | ", out);
                print!("\x1b[31mFAIL\x1b[0m\n");
            }
        }
    }

    #[test]
    fn test_make_two_arrays_equal_by_reversing_sub_arrays_1460() {
        let tests = TestCase::new(|p: (Vec<i32>, Vec<i32>)|
            solutions::make_two_arrays_equal_by_reversing_sub_arrays_1460::can_be_equal(p.0, p.1)
        );

        tests.run_test((vec![1, 2, 3, 4], vec![2, 4, 1, 3]), true);
        tests.run_test((vec![7], vec![7]), true);
        tests.run_test((vec![3, 7, 9], vec![3, 7, 11]), false);
    }

    #[test]
    fn test_maximum_product_of_two_elements_in_an_array_1464() {
        let tests = TestCase::new(
            solutions::maximum_product_of_two_elements_in_an_array_1464::max_product
        );

        tests.run_test(vec![3, 4, 5, 2], 12);
        tests.run_test(vec![1, 5, 4, 5], 16);
        tests.run_test(vec![3, 7], 12);
    }
}
