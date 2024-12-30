// This program is in F#, the .Net version of OCaml: get from fsharp.org

module Lambdacalc
open System;
(*
   To load script into interactive interpreter:
   start fsharpi/fsi
   #load "lambdacalc.fsx";;
   open Lambdacalc;;

   lambda x.A is written fun x -> A in F#
   lambda x.lambda y.A is fun x y -> A, or fun x -> fun y -> A

   F# contains built-in tuples (a,b) and built-in fixedpoint operator
   (let rec) for recursion.
*)

// Pure Typed Lambda Calculus in F#
// The typed lambda calculus is not as expressive as the untyped on without
// additional constructs, though many terms can still be defined.  The
// limitation is in how they can be used.

let I = fun x -> x;  // lambda x.x, see inferred type below
let K = fun x y -> x; 
let S = fun x y z -> x z (y z);

// Note that fun x y -> y is same as fun x -> fun y -> x, but not the same
// as fun (x,y) -> y.  The (,) indicates a PAIR, which is built-in to F#.
// All functions are "Curried" by default.

let TRUE = K;
let ZERO x = K I x;  // must write this way instead of let ZERO=K I
let FALSE = ZERO;

// The typed version of if-else implies that the true case and the false
// case must be of the same type.

// call-by-value if-else
let IFELSE0 = fun a b c -> (a b c);
// simulated call-by-name if-else
let IFELSE = fun a b c -> (a b c)(); // apply dummy lambda

IFELSE TRUE (fun ()-> 1) (fun() -> 1/0);; // doesn't crash

// boolean operators:
let AND = fun x y -> x y FALSE
let OR = fun x y -> x TRUE y
let NOT = fun x -> x FALSE TRUE

// cons pairs - but no nested pairs without algebraic data type
// the car and the cdr must have the same type.
let CONS = fun x y -> fun s -> s x y; // same as fun x y s -> s x y
let CAR = fun p -> p TRUE
let CDR = fun p -> p FALSE
let NIL = ZERO;
let ISNIL = fun p -> p (fun x y z -> FALSE) TRUE;

// Church Numerals
let church2 = fun f x -> f (f x);
let church3 = fun f x -> f (f (f x));
let ONE = fun f x -> f x;
let SUCC = fun n f x -> n f (f x);
let ADD = fun m n f x -> m f (n f x);
let TIMES = fun m n f x -> m (n f) x;
let EXPT = fun m n -> n m;
let ISZERO = fun n -> n (fun x -> FALSE) TRUE
let BASE x = CONS ZERO ZERO x
let NEXT = fun p -> CONS (CDR p) (SUCC (CDR p))
let PRED = fun n -> CAR (n NEXT BASE)
let SUBTRACT = fun m n -> n PRED m;

// Recursion, unlike in the untyped-case, must be added to the typed
// lambda calculus as an external (extra) constant:

let rec FIX M = M (FIX M);

// Note that FIX can be defined because F# already supports recursive
// definitions, so it already has something equivalent built-in.

(*  Inferred Types:  (printed up on #load "lambdacalc.fsx")

  val I : x:'a -> 'a
  val K : x:'a -> y:'b -> 'a
  val S : x:('a -> 'b -> 'c) -> y:('a -> 'b) -> z:'a -> 'c
  val TRUE : ('a -> 'b -> 'a)
  val ZERO : x:'a -> ('b -> 'b)
  val FALSE : ('a -> 'b -> 'b)
  val IFELSE0 : a:('a -> 'b -> 'c) -> b:'a -> c:'b -> 'c
  val IFELSE : a:('a -> 'b -> unit -> 'c) -> b:'a -> c:'b -> 'c
  val AND : x:('a -> ('b -> 'c -> 'c) -> 'd) -> y:'a -> 'd
  val OR : x:(('a -> 'b -> 'a) -> 'c -> 'd) -> y:'c -> 'd
  val NOT : x:(('a -> 'b -> 'b) -> ('c -> 'd -> 'c) -> 'e) -> 'e
  val CONS : x:'a -> y:'b -> s:('a -> 'b -> 'c) -> 'c
  val CAR : p:(('a -> 'b -> 'a) -> 'c) -> 'c
  val CDR : p:(('a -> 'b -> 'b) -> 'c) -> 'c
  val NIL : ('a -> 'b -> 'b)
  val ISNIL :
    p:(('a -> 'b -> 'c -> 'd -> 'e -> 'e) -> ('f -> 'g -> 'f) -> 'h) -> 'h       
  val church2 : f:('a -> 'a) -> x:'a -> 'a
  val church3 : f:('a -> 'a) -> x:'a -> 'a
  val ONE : f:('a -> 'b) -> x:'a -> 'b
  val SUCC : n:(('a -> 'b) -> 'b -> 'c) -> f:('a -> 'b) -> x:'a -> 'c
  val ADD : m:('a -> 'b -> 'c) -> n:('a -> 'd -> 'b) -> f:'a -> x:'d -> 'c
  val TIMES : m:('a -> 'b -> 'c) -> n:('d -> 'a) -> f:'d -> x:'b -> 'c
  val EXPT : m:'a -> n:('a -> 'b) -> 'b
  val ISZERO : n:(('a -> 'b -> 'c -> 'c) -> ('d -> 'e -> 'd) -> 'f) -> 'f        
  val FIX : M:('a -> 'a) -> 'a

  // functions that don't take arguments (not pure lambda calculus) are
  // of type unit -> .. and those that don't return values are of type
  // .. -> unit.
*)


//////

(*
// convert built-in numbers to church numerals
let rec tochurch n =
  match n with
    | 0 -> ZERO
    | n -> SUCC (tochurch (n-1));;

let fromchurch n = n (fun x -> x+1) 0;

let church5 = tochurch 5
let fromchurch5 = fromchurch church5
let nottrue = fromchurch ZERO
let notfalse = fromchurch ONE
let r x = ISZERO ZERO x
//printfn "%d" fromchurch5;  // prints 5
//printfn "%d" (fromchurch (fun x-> ISZERO ZERO x))
let tobool x = x true false;
printfn "%A" (tobool TRUE);
printfn "%A" (tobool FALSE);
let frombool x = if x then TRUE else FALSE;
printfn "%A" (tobool r);
let r2 = ISZERO ONE;
printfn "%A" (tobool r2);
*)

//// Since the CONS can only define pairs with CAR and CDR of the same
//// type, an algebraic type is required to define data structures such
//// as linked lists:

type Llist<'T> = EmptyList | Cons of 'T*Llist<'T>

// Here, 'T is a quantified type variable.

let m = Cons(2,Cons(3,Cons(5,EmptyList)));  // has type Llist<int>
