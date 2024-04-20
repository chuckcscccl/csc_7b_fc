#![allow(dead_code)]
use csc_7b_fc::*;
use csc_7b_fc::Expr::*;


pub fn compile(e:&Expr) {
  match e {
    Val(x) => println!("push {}",x),
    Neg(a) => {
       compile(a);
       println!("push 0");
       println!("pop bx");     // moves 0 into bx register
       println!("pop ax");     // load result for a into ax
       println!("sub ax bx");  // this instruction has semantics bx -= ax
       println!("push bx");    // always push result (0-a) on stack 
    },
    _ => {
       eprintln!("ERROR: COMPILATION FAILED;");
       return;
   },
  }//match
}//compile



fn main()  // this version of main compiles
{
   //print!("Enter Expression: ");  // don't echo to output
   let mut input = String::from("0");
   std::env::args().nth(1).map(|s|{ input = String::from(s); });
   let tokens = lex(&input[..].trim());
   let exp = parse(&tokens);
   exp.map(|e|{ compile(&e); });
}//main   
