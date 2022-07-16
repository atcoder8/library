/// Module for testing
pub mod random_test {
    use std::io::{self, Write};

    /// Tuple of input data
    #[derive(Debug, Clone)]
    pub struct Input();

    /// Tuple of Output data
    #[derive(Debug, PartialEq, Eq)]
    pub struct Output();

    /// Perform the specified number of tests.
    /// Returns true only if all tests passed.
    pub fn test() -> bool {
        const NUMBER_OF_TESTS: usize = 1000;

        for test_case_index in 0..NUMBER_OF_TESTS {
            let input = generator();
            let jury_output = jury(input.clone());
            let solve_output = solve(input.clone());

            if jury_output != solve_output {
                eprint!(
                    "\
Wrong Answer on Test #{}

[Input]
{:?}

[Output(Jury)]
{:?}

[Output(Solve)]
{:?}
",
                    test_case_index, input, jury_output, solve_output
                );
                io::stderr().flush().unwrap();

                return false;
            }
        }

        eprintln!("No problem found.");
        io::stderr().flush().unwrap();

        true
    }

    /// Generate test cases.
    fn generator() -> Input {
        // let rng = rand::thread_rng();

        Input()
    }

    /// Returns the correct answer.
    fn jury(input: Input) -> Output {
        // Unpacking input.
        let Input() = input;

        Output()
    }

    /// Test this program.
    pub fn solve(input: Input) -> Output {
        // Unpacking input.
        let Input() = input;

        Output()
    }
}
