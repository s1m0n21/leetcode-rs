use std::cmp::PartialEq;
use std::fmt::Debug;
use std::time::Instant;

pub struct TestCase<I, E>
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
    #[inline]
    pub fn new(f: fn(I) -> E) -> TestCase<I, E> {
        TestCase { func: f }
    }

    pub fn run_test(&self, i: I, e: E) {
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
