// Eytzinger layout AVL trees (immutable once constructed, for now)
#![allow(dead_code)]
#![allow(non_snake_case)]
#![allow(unused_variables)]
use std::mem;
use crate::avltree::Bst::*;
use crate::avltree::*;
use std::collections::VecDeque;

fn left(i:usize) -> usize {2*i+1}
fn right(i:usize) -> usize {2*i+2}
fn parent(i:usize) -> usize { (i-1)/2 }  // no underflow check!

//2**n
const fn power2(mut n:usize) -> usize {
  let mut ax = 1;
  let mut fct = 2;
  while n>0 {
    if n%2==1 {ax*= fct;}
    fct = fct*fct;
    n = n/2;
  }
  ax
}

fn cmp<T:Ord>(x:&T, y:&T) -> i8 {
  if x==y { 0}
  else if x<y {-1}
  else {1}
}

#[derive(Clone, Debug)]
pub struct Eytzinger<T> {
  nodes: Vec<Option<T>>,
  size: usize,
}
impl<T:Ord> Eytzinger<T> {
  pub fn len(&self) -> usize { self.size }
  pub fn new() -> Self {
    Eytzinger {
      nodes: Vec::new(),
      size: 0,
    }
  }

  pub fn with_capacity(cap:usize) -> Self {
    let mut n = Vec::with_capacity(cap);
    n.resize_with(cap,||None);
    Eytzinger {
      nodes: n,
      size: 0,
    }
  }

  pub fn search(&self, x:&T) -> bool {
    let mut current = 0; // root index
    while (current<self.nodes.len()) {
       let c = self.nodes[current].as_ref()
         .map(|item|cmp(item,x))
	 .unwrap_or(-2);
       if (c == -2) { return false; }
       else if (c == 0) { return true; }
       else if (c == -1) { current = left(current); }
       else { current = right(current);}
    }
    false
  }

  fn from_bstr(&mut self, tree: Bst<T>, index:usize) {
    if index>=self.nodes.len() || self.nodes[index].is_none() { return; }
    match tree {
      Empty => { /* self.nodes[index] = None; */},  // stays None
      Node(cell) => {
       self.nodes[index] = Some(cell.item);
       self.size += 1;
       self.from_bstr(cell.left, left(index));
       self.from_bstr(cell.right, right(index));       
      }
    }//match
  }//from_bst

  pub fn from_bst(tree:Bst<T>) -> Self {
    let cap = power2(tree.height() as usize);
    let mut newself = Eytzinger::with_capacity(cap);
    newself.from_bstr(tree, 0);
    newself
  }
  
}//main impl
