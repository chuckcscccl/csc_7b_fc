//!  #   CSC 123/252 Assignment: compiling simple expression trees.
//!
//! Abstract Machine *AM16* is a simple instruction set architecture that
//! can compute arithmetic expressions as defined by the **enum [Expr]** AST
//! type in this module (see source code).  The abstract machine has
//! the following general purpose registers:
//!    **ax, bx, cx, dx**.
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
//! four registers, while the src (source) operand can be register or
//! immediate (constant).  The semantics of an ALU instruction such as
//! `sub ax bx` is `bx -= ax`.  In addition, the div (divide) instruction has
//! a special semantics: it calculates both the quotient and the remainder.
//! The quotient is stored in the dst register while the remainder is always
//! stored in register cx.  If the dst register is also cx, then the quotient
//! is discarded.  For example, if ax contains value 9 and bx contains value 2
//! then executing `div bx ax` will store 4 in ax and 1 in cx.
//!
//! For example, the following program computes 5-3:
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
//! 2+3*4, which can be computed with:
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
//! final result on top of the stack.  I've written a skeleton with the case
//! for `Neg(a)` and you just need to finish the other cases.  
//!
//! This part of the assignment is not meant to be difficult.

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
    }//match
  }//is_token

  /// cloning the entire tree is expensive but a token is shallow and can
  /// be copied.
  pub fn clone_token(&self) -> Self {
    match self {
      Val(n) => Val(*n),
      Sym(c) => Sym(*c),
      EOF => EOF,
      _ => Dummy,    // everything else clones to Dummy
    }//match
  }//clone_token

}  // method-style implementations

fn proper(e: &Expr) -> bool {
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
            },
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
            }
            [cdr @ .., Sym('-'), e1] if prec(lookahead, 'u') => {
                let e = Neg(Box::new(stack.pop().unwrap()));
                stack[sl - 2] = e; // e moved to stack
            }
            _ if ti+1 < tokens.len() => {
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