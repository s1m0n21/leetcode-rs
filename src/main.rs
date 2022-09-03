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

    #[test]
    fn test_shortest_supersequence_lcci_17_18() {
        let tests = TestCase::new(|p: (Vec<i32>, Vec<i32>)|
            solutions::shortest_supersequence_lcci_17_18::shortest_seq(p.0, p.1)
        );

        tests.run_test((vec![7, 5, 9, 0, 2, 1, 3, 5, 7, 9, 1, 1, 5, 8, 8, 9, 7], vec![1, 5, 9]), vec![7, 10]);
        tests.run_test((vec![1, 2, 3], vec![4]), vec![]);
        tests.run_test((vec![], vec![4]), vec![]);
    }

    #[test]
    fn test_shuffle_the_array_1470() {
        let tests = TestCase::new(|p: (Vec<i32>, i32)|
            solutions::shuffle_the_array_1470::shuffle(p.0, p.1)
        );

        tests.run_test((vec![2, 5, 1, 3, 4, 7], 3), vec![2, 3, 5, 4, 1, 7]);
        tests.run_test((vec![1, 2, 3, 4, 4, 3, 2, 1], 4), vec![1, 4, 2, 3, 3, 2, 4, 1]);
        tests.run_test((vec![1, 1, 2, 2], 2), vec![1, 2, 1, 2]);
    }

    #[test]
    fn test_validate_stack_sequences_946() {
        let tests = TestCase::new(|p: (Vec<i32>, Vec<i32>)|
            solutions::validate_stack_sequences_946::validate_stack_sequences(p.0, p.1)
        );

        tests.run_test((vec![1, 2, 3, 4, 5], vec![4, 5, 3, 2, 1]), true);
        tests.run_test((vec![1, 2, 3, 4, 5], vec![4, 5, 3, 1, 2]), false);
    }

    #[test]
    fn test_final_prices_with_a_special_discount_in_a_shop_1475() {
        let tests = TestCase::new(
            solutions::final_prices_with_a_special_discount_in_a_shop_1475::final_prices
        );

        tests.run_test(vec![8, 4, 6, 2, 3], vec![4, 2, 4, 2, 3]);
        tests.run_test(vec![1, 2, 3, 4, 5], vec![1, 2, 3, 4, 5]);
        tests.run_test(vec![10, 1, 1, 6], vec![9, 0, 1, 6]);
    }

    #[test]
    fn test_maximum_length_of_pair_chain_646() {
        let tests = TestCase::new(
            solutions::maximum_length_of_pair_chain_646::find_longest_chain
        );

        tests.run_test(vec![vec![1, 2], vec![2, 3], vec![3, 4]], 2);
    }
}
