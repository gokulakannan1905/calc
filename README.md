# g_calc
Simple calculator utility written in rust

## Installation

Add this to your Cargo.toml:

    [dependencies]
    g_calc = "0.1.0"

## Usage

    use g_calc::{convert,solve};

    let sample_expr = "1 + 2 * 3";

    //this fn converts the given infix expression to postfix expression
    let postfix_expr = g_calc::convert(sample_expr).unwrap();

    //this fn solves the postfix expression and returns the result
    let output = g_calc::solve(postfix_expr).unwrap();
    // output = 7    

> *works for basic math calculations*

- exponentiation ( ^ )
- add, substract, multiplication, division
- supports decimal notation ( eg., 0.1 + 0.2)
- parentheses support ()