//!  #   CSC 123/252 Assignment: compiling simple expression trees.
//!
//! Abstract Machine *AM16* is a 16-bit instruction set architecture that
//! can compute arithmetic expressions as defined by the **enum [Expr]** AST
//! type in this module (see source code).  The abstract machine has
//! the following general purpose registers:
//!    **ax, bx, cx**.
//!
//! And the following instructions
//! ```
//!    push src
//!    pop dst
//!    add src dst
//!    sub src dst
//!    mult src dst
//!    div src dst
//! ```
//! In each instruction, the dst (destination) operand must name one of the
//! three registers, while the src (source) operand can be register or
//! immediate (constant).  The semantics of an ALU instruction such as
//! `sub ax bx` is `bx -= ax`.  In addition, the div (divide) instruction has
//! a special semantics: it calculates both the quotient and the remainder.
//! The quotient is stored in the dst register while the remainder is always
//! stored in register cx.  If the dst register is also cx, then the quotient
//! is discarded.  For example, if ax contains value 9 and bx contains value 2
//! then executing `div bx ax` will store 4 in ax and 1 in cx.
//!
//! The following sample program computes 5-3:
//! ```
//!   push 3
//!   push 5
//!   pop ax
//!   pop bx
//!   sub bx ax
//!   push ax
//! ```
//! The last instruction should always push the result on the stack: this
//! protocol is essential in order to compute compound expressions such as
//! 2+3*4, which compiles to:
//! ```
//!   push 4
//!   push 3
//!   pop ax
//!   pop bx
//!   mult ax bx
//!   push bx
//!   push 2
//!   pop ax
//!   pop bx
//!   add ax bx
//!   push bx
//! ```
//! At the end of a successful computation, the top of the stack always
//! contains the result.
//!
//! Your assignment is to write a compiler for simple arithmetic expressions.
//! It should print the sequence of instructions, which can be captured into
//! a file and executed on "vm16", a virtual machine implementation of AM16
//! written in C++.
//!
//! To compile the program, execute a POSTORDER traveral on the expression tree.
//! The leaves of the tree contain constants of the form Val(n) in abstract
//! syntax.  For such a leaf, just `println!("push {}",n);`  For a non-leaf
//! node such as `Plus(a,b)`, you need to recursively compile a and b first
//! (postorder) then add instructions to calculate the sum, then push the
//! final result on top of the stack.  I've written a [skeleton](https://github.com/chuckcscccl/csc_7b_fc/blob/main/src/main.rs) with the case
//! for `Neg(a)` and you just need to finish the other cases.  
//!
//! <p>
//!
//!  -----------------
//! This assignment's base program is hosted on **[github](https://github.com/chuckcscccl/csc_7b_fc/)**.  The assignment is designed to be a gentle introduction to Rust
//! because you're mostly just going to [println]!.  Do the following to
//! set up a Rust "cargo" project, assuming you've rust already installed.
//!   1. `git clone https://github.com/chuckcscccl/csc_7b_fc.git`
//!
//!      If you don't have git, goto the github link and "get" it manually.
//!      Periodically, I will update this repository and you will need to
//!      do a `git pull` inside the folder to upgrade.
//!
//!   2. `cargo new myfirstcrate`  
//!
//!      This creates a cargo project folder.  Make sure this
//!      folder is in the same folder as `csc_7b_fc`.  Inside `src/main.rs`
//!      there's a main that prints hello world.  You will replace this main.
//!      If you submit a project that prints hello world you will lose
//!      one million points.
//!
//!   3. Go into the `myfirstcrate` folder and edit `Cargo.toml`. Add the following under `[dependencies]`:
//!      ```
//!        [dependencies]
//!        csc_7b_fc = { path = "../csc_7b_fc/" }
//!      ```
//!      Change the path if that's not where the base crate is located.
//!
//!   4. `cargo build`  : this compiles the program and its dependencies.
//!
//!   5. `cargo run`  : this runs the program
//!
//!   6. Go into the folder `csc_7b_fc/vm16/` and read the [README](https://github.com/chuckcscccl/csc_7b_fc/tree/main/vm16).  Compile and test the `vm16` program.  You can then move the vm16 executable into your project's folder so
//!   it's more accessible.
//!
//!   7. Rewrite the `main.rs` in `myfirstcrate/src/`.  I've written a **[skeleton](https://github.com/chuckcscccl/csc_7b_fc/blob/main/src/main.rs)**
//!   for you that you can edit/follow.  Make sure this main is in YOUR crate.
//!   Do not change my crate (`csc_7b_fc`).  Test it by compiling expressions
//!   such as 3*20-9%2.  Run it on vm16.
//!   
//!   8. Submit only the main.rs of your project.
//!   <p>
//!
//! There are additional helpful commands you can run with [cargo](https://doc.rust-lang.org/cargo/):
//!   1. `cargo fmt` : this will format your code to be nicely spaced and
//!   properly indented.
//!   2. `cargo doc` : generate documentation from certain types of comments
//!   written in markdown
//!   (see source code for example).  Documentation should be provided for
//!   all `pub` items.  The docs are viewable in `/target/doc/myfirstcrate/index.html`
//!   3. `cargo clipply` : runs addtional static analysis on your code, provides suggestions such as how to [improve performance](https://nnethercote.github.io/perf-book/linting.html).
//!  <p>
//!
//! What follows is the documentation on the different elements of this crate,
//! generated from comments.
//!
//!  -----------------

#![allow(dead_code)]
#![allow(non_snake_case)]
#![allow(unused_variables)]
#![allow(unused_parens)]
#![allow(unused_imports)]
#![allow(non_camel_case_types)]
use std::fmt::{Display, Formatter, Result};
use std::io::{self, Read, Write};
use Expr::*;
// online calculator in Rust, with shift-reduce parser

/// Abstract Syntax (AST) type for arithmetic expressions. Smart pointer
/// [Box] is required to define recursive structures.  One drawback of
/// Rust is that Box blocks nested pattern matching, so sometimes
/// nested `match` expressions are required.
#[derive(Debug)]
pub enum Expr {
    Val(i32),                   // i32 is type for 32 bit signed ints
    Plus(Box<Expr>, Box<Expr>), // recursion requires smart pointer
    Minus(Box<Expr>, Box<Expr>),
    Times(Box<Expr>, Box<Expr>), // Box is like unique_ptr in C++
    Divide(Box<Expr>, Box<Expr>),
    Mod(Box<Expr>, Box<Expr>),
    Neg(Box<Expr>),
    Sym(char),
    EOF,
    Dummy,
} // Expr enum

impl Expr {
    /// alias to [eval] function, but called as a method: `expr1.eval_to()`
    pub fn eval_to(&self) -> Option<i32> {
        eval(self)
    }

    /// determines if expr is a shallow token, produced by the lexical
    /// tokenizer before parsing.
    pub fn is_token(&self) -> bool {
        match self {
            Val(_) | Sym(_) | EOF | Dummy => true,
            _ => false,
        } //match
    } //is_token

    /// cloning the entire tree is expensive but a token is shallow and can
    /// be copied.  Non-token expressions are cloned to `Dummy`.
    pub fn clone_token(&self) -> Self {
        match self {
            Val(n) => Val(*n),
            Sym(c) => Sym(*c),
            EOF => EOF,
            _ => Dummy, // everything else clones to Dummy
        } //match
    } //clone_token
} // method-style implementations

/// checks if expr is a proper AST expression, and not just a token pre-parsing.
/// This function is not equivalent to `!.is_token` because `Val(_)` is both a
/// token and a proper expression.
pub fn proper(e: &Expr) -> bool {
    match e {
        Sym(_) | EOF | Dummy => false,
        _ => true,
    }
} //proper

/// Eval function evaluates to an [Option] type. Further demonstrates
/// monadic error handling, which is the only form of error handling
/// in Rust.
pub fn eval(e: &Expr) -> Option<i32> {
    match e {
        Val(x) => Some(*x),                // x is a ref so has to be deref'ed
        Neg(x) => eval(x).map(|a| -1 * a), //& does deref coercion on Box
        Plus(x, y) => eval(x).zip(eval(y)).map(|(a, b)| a + b),
        Minus(x, y) => eval(x).zip(eval(y)).map(|(a, b)| a - b),
        Times(x, y) => eval(x).zip(eval(y)).map(|(a, b)| a * b),
        Divide(x, y) => eval(y).and_then(|b| if b != 0 { eval(x).map(|a| a / b) } else { None }),
        Mod(x, y) => eval(y).and_then(|b| if b != 0 { eval(x).map(|a| a % b) } else { None }),
        _ => None,
    } //match
} //eval

/////////// Trait implementations for Expr

impl Default for Expr {
    fn default() -> Expr {
        Dummy
    }
}

impl Display for Expr {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result // required by trait
    {
        match self {
            Val(x) => write!(f, "{}", x),
            Plus(x, y) => write!(f, "({}+{})", x, y),
            Times(x, y) => write!(f, "{}*{}", x, y),
            Minus(x, y) => write!(f, "({}-{})", x, y),
            Divide(x, y) => write!(f, "{}/{}", x, y),
            Mod(x, y) => write!(f, "{}%{}", x, y),
            Neg(x) => {
                if let Neg(y) = &**x {
                    write!(f, "{}", y)
                } else {
                    write!(f, "-{}", x)
                }
            }
            Sym(s) => write!(f, " {} ", s),
            EOF => write!(f, " EOF "),
            Dummy => write!(f, " Dummy "),
        } //match
    }
} // impl Display for Expr
  // no ability to deep-pattern match inside a Box/Rc

/// Function that prompts user for expression, parses and evaluates it.
pub fn eval_interactive() // interpreter
{
    print!("Enter Expression: ");
    io::stdout().flush().unwrap();
    let mut input = String::new();
    if let Ok(_) = io::stdin().read_line(&mut input) {
        let tokens = lex(&input[0..input.len() - 1].trim());
        //println!("tokens: {:?}",&tokens);
        let exp = parse(&tokens);
        exp.and_then(|e| {
            eval(&e).map(|n| {
                println!("Value of {} = {}", &e, n);
            })
        });
    } // if let Ok(n)
} // - evals instead of compiles

////////////////////////////////////////////////////////////////////////////
/// Simple String Tokenizer.  Takes a string slice and generates a [Vec]tor
/// of tokens such as `Sym("+")`, `Val(3)`, etc.
pub fn lex(inp: &str) -> Vec<Expr> {
    let input: Vec<char> = inp.chars().collect();
    let mut tokens: Vec<Expr> = Vec::new();
    let mut i: usize = 0;
    while i < input.len() {
        let mut c = input[i];
        i += 1; // get one char
        let mut isnum = false;
        let mut n = 0;
        while c.is_digit(10) && i <= input.len() {
            isnum = true;
            n = n * 10 + c.to_digit(10).unwrap();
            if i < input.len() {
                c = input[i];
                i += 1;
            } else {
                i += 1;
            }
        } // while numerical digit
        if isnum {
            tokens.push(Val(n as i32));
        }
        if !c.is_digit(10) && c != ' ' {
            tokens.push(Sym(c));
        }
    } //while
    tokens.push(EOF);
    return tokens;
} //lexer

/// operator precedence parser
fn precedence(e: &Expr) -> u32 {
    match e {
        Val(_) => 250,
        Sym('+') => 100,
        Sym('-') => 100,
        Sym('*') => 200,
        Sym('/') => 200,
        Sym('%') => 200,
        Sym('u') => 220,
        Sym('(') => 500,
        Sym(')') => 10,
        EOF => 5,
        _ => 0,
    }
}
fn prec(a: &Expr, b: char) -> bool {
    precedence(a) <= precedence(&Sym(b))
}
// assume universal left-associativity.

fn ateof(e: &Expr) -> bool // at end-of-file predicate
{
    match e {
        EOF => true,
        _ => false,
    }
}

/// This function takes a vector of tokens produced by the [lex] function
/// and returns an Expr inside an Option: Some(AST) or None if parsing
/// failed. The function is defined using "slice patterns". Also note
/// the while loop: recursion is generally discouraged in Rust.  However,
/// in dealing with recursive Expr trees it is OK because these trees will
/// not be large.
pub fn parse(tokens: &Vec<Expr>) -> Option<Expr> {
    let mut stack: Vec<Expr> = Vec::new();
    let mut ti: usize = 0; // indexes tokens
    let mut lookahead = &tokens[ti];
    while !(ateof(lookahead) && stack.len() == 1) {
        let sl = stack.len();
        match stack.as_slice() {
            // match against stack as slice
            [cdr @ .., Sym('('), e, Sym(')')] if prec(lookahead, '(') => {
                stack.swap(sl - 2, sl - 3); // move e down stack
                stack.truncate(sl - 2); // pop last two values
            }
            [cdr @ .., e1, Sym('+'), e2] if prec(lookahead, '+') => {
                let mut tos = stack.split_off(sl - 3);
                let b = Box::new(tos.remove(2));
                let a = Box::new(tos.remove(0));
                stack.push(Plus(a, b));
            }
            [cdr @ .., e1, Sym('-'), e2] if prec(lookahead, '-') => {
                let mut tos = stack.split_off(sl - 3);
                let b = Box::new(tos.remove(2));
                let a = Box::new(tos.remove(0));
                stack.push(Minus(a, b));
            }
            [cdr @ .., e1, Sym('*'), e2] if prec(lookahead, '*') => {
                let mut tos = stack.split_off(sl - 3);
                let b = Box::new(tos.remove(2));
                let a = Box::new(tos.remove(0));
                stack.push(Times(a, b));
            }
            [cdr @ .., e1, Sym('/'), e2] if prec(lookahead, '/') => {
                let mut tos = stack.split_off(sl - 3);
                let b = Box::new(tos.remove(2));
                let a = Box::new(tos.remove(0));
                stack.push(Divide(a, b));
            }
            [cdr @ .., e1, Sym('%'), e2] if prec(lookahead, '%') => {
                let mut tos = stack.split_off(sl - 3);
                let b = Box::new(tos.remove(2));
                let a = Box::new(tos.remove(0));
                stack.push(Mod(a, b));
            },
            [cdr @ .., e2, Sym('-'), e1] if !proper(e2) && prec(lookahead, 'u') => {
                let e = Neg(Box::new(stack.pop().unwrap()));
                stack[sl - 2] = e; // e moved to stack
            },
            [Sym('-'), e1] if prec(lookahead, 'u') => {
                let e = Neg(Box::new(stack.pop().unwrap()));
                stack[sl - 2] = e; // e moved to stack
            },
            _ if ti + 1 < tokens.len() => {
                // shift
                stack.push(lookahead.clone_token());
                ti += 1;
                lookahead = &tokens[ti];
            }
            _ => {
                for ex in stack {
                    print!("{:?}::", ex);
                }
                eprintln!("\nPARSING ERROR");
                return None;
            }
        } // match
    } // while
    return stack.pop();
} //parse

///////////// bijective map
pub mod bijectivemap;

///////////// circular queue
pub mod circularqueue;

//////////// AVL Tree
pub mod avlmap;
pub mod avltree;
pub mod avlnavigator;
pub mod eytzinger;

pub mod redblack;

pub mod twoway;
