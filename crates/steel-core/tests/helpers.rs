extern crate steel;

use std::fs::File;
use std::io::{BufRead, BufReader};
// use steel::interpreter::evaluator::Evaluator;
// use steel::parser::*;
use steel::PRELUDE;
use steel::{steel_vm::engine::Engine, SteelVal};

// use std::collections::HashMap;

/// test to make sure file used as input leads to output
pub fn test_from_files(input_path: &str, output_path: &str) {
    let inputfile = File::open(input_path).unwrap();
    let outputfile = File::open(output_path).unwrap();
    test_lines(BufReader::new(inputfile), BufReader::new(outputfile));
}

pub fn test_lines(input: impl BufRead, output: impl BufRead) {
    let mut evaluator = Engine::new();
    evaluator.compile_and_run_raw_program(PRELUDE).unwrap();

    let io_lines = input.lines().zip(output.lines());
    for (line_in, line_out) in io_lines {
        let line_in = line_in.unwrap();
        let line_out = line_out.unwrap();
        test_line(&line_in, &[&line_out], &mut evaluator);
    }
}

// TODO -> clean this up
// the many references are broken
pub fn test_line(input: &str, output: &[&str], evaluator: &mut Engine) {
    let result = evaluator.compile_and_run_raw_program(input);
    match result {
        Ok(vals) => {
            println!("Expected values: {output:?}");
            println!("Resulting values: {vals:?}");

            let vals: Vec<_> = vals
                .into_iter()
                .filter(|x| x.clone() != SteelVal::Void)
                .collect();

            let output: Vec<&&str> = output.iter().filter(|x| **x != "#<void>").collect();

            println!("post - Expected values: {output:?}");
            println!("post - Resulting values: {vals:?}");

            // TODO -> this shouldn't check this here
            // voids should be skipped if the outputs don't match
            // as in -> leading voids can be skipped
            assert_eq!(output.len(), vals.len());

            for (expr, &&expected) in vals.iter().zip(output.iter()) {
                println!("expr to string: {:?}", expr.to_string());
                println!("expected to string: {expected:?}");

                assert_eq!(expr.to_string(), expected);
                // match expr {
                //     Ok(x) => assert_eq!(x.to_string(), expected),
                //     Err(x) => assert_eq!(x.to_string(), expected),
                // }
            }
        }
        Err(e) => assert_eq!(e.to_string(), output[0]),
    }
}
