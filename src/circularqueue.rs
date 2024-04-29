/// A circular queue using a vector of options underneath.  Can't use an array
/// because the size of the array is part of its type in Rust.  The
/// unused portions of the vector will hold value None.  Look at the
/// source code for details.  This program was based on a
/// roughly equivalent **[C++ Version]**
pub struct CircularQueue<T, const INITCAP:usize = 64> {
  q: Vec<Option<T>>,
  front: usize,
  size: usize,
}

impl<T, const INITCAP:usize> CircularQueue<T,INITCAP> {
  /// number of values in queue.  This function can be called from a const
  /// context, which means it can be evaluated by the compiler, although
  /// this is unlikely since vectors are heap-allocated.
  pub const fn len(&self) -> usize { self.size }

  /// current capacity of queue
  pub fn capacity(&self) -> usize { self.q.len() }

  /// creates an empty queue with default initial capacity defined by
  /// const generic INITCAP
  pub fn new() -> Self {  //constructor Self = CircularQueue
    let mut v = Vec::with_capacity(INITCAP);
    // let mut vec = [None;INITCAP]  won't compile : can't copy/clone
    v.resize_with(INITCAP,||None); // loop underneath
    //assert_eq!(INITCAP, v.len());
    CircularQueue {
      front : 0,
      size : 0,
      q : v
    }
  }//new

  fn index(&self, i:usize) -> usize {
    (self.front + i) % self.q.len()
  }  // converts logical index into actual index

  fn resize(&mut self) {  // double capacity, moves queue
    let newcap = self.q.len()*2;
    let mut newq = Vec::with_capacity(newcap);
    newq.resize_with(newcap,||None);
    let size = self.size;
    for i in 0..size {
      let k = self.index(i);
      std::mem::swap(&mut self.q[k], &mut newq[i]);
    }
    self.q = newq; // always move
    self.front = 0;
  }//resize

  /// inserts a value at the back of the queue, wrapping around to the right
  /// if necessary.  Increases capacity if necessary.
  pub fn push_back(&mut self, x:T) {
    if self.size>= self.q.len() {self.resize();}
    let back = self.index(self.size);
    self.q[back] = Some(x);  // move into vector is ok
    self.size+=1;
  }

  /// inserts a value at the front of the queue, wrapping around to the left
  /// if necessary. Increases capacity if necessary.
  pub fn push_front(&mut self, x:T) {
    if self.size>= self.q.len() {self.resize();}
    let newfront = self.index(self.q.len() - 1);
    self.q[newfront] = Some(x);
    self.front = newfront;
    self.size += 1;
  }

  /// returns (moves) value at back of the queue, if it exists.
  pub fn pop_back(&mut self) -> Option<T> {
    if self.size==0 { return None; }
    let mut answer = None;
    let last = self.index(self.size-1);
    std::mem::swap(&mut answer, &mut self.q[last]);
    self.size -= 1;
    answer
  }

  /// returns (moves) value at front of the queue, if it exists.
  pub fn pop_front(&mut self) -> Option<T> {
    if self.size==0 { return None; }
    let mut answer = None;
    let first = self.index(0);
    std::mem::swap(&mut answer, &mut self.q[first]);
    self.front = self.index(1);
    self.size -= 1;
    answer
  }

  /// Maps mutable closure f over each value of the queue.  The
  /// closure has the ability to mutate each value as well as
  /// mutate its environment (see the source for the `test_main` function).
  pub fn mapfun<F:FnMut(&mut T)>(&mut self, mut f:F) {
    for i in 0..self.size {
      let k = self.index(i);
      f(self.q[k].as_mut().unwrap());
    }
  }

  /// returns an immutable iterator, allows `for x in queue.iter()`
  pub fn iter<'lt>(&'lt self) -> Cqiter<'lt,T,INITCAP> {
    Cqiter {
       cq : self,
       index : 0,
    }
  }//iter
  
} // impl circularqueue


// overloading [i] only possible by implementing a trait:
use std::ops::{Index, IndexMut};

impl<T, const C:usize> Index<usize> for CircularQueue<T,C> {
  type Output = T;
  fn index(&self, i:usize) -> &Self::Output {
    let k = self.index(i);
    self.q[k].as_ref().unwrap()  
  }
}

impl<T, const C:usize> IndexMut<usize> for CircularQueue<T,C> {
  fn index_mut(&mut self, i:usize) -> &mut Self::Output {
    let k = self.index(i);
    self.q[k].as_mut().unwrap()  
  }
}

/// Iterator type for circular queues. This is the structure that we will
/// implement the [Iterator] trait for.
pub struct Cqiter<'lt,T, const C:usize> {
  cq : &'lt CircularQueue<T,C>,
  index : usize, // current index
}
impl<'lt,T, const C:usize> Iterator for Cqiter<'lt,T,C> {
  type Item = &'lt T;
  fn next(&mut self) -> Option<Self::Item> {
    if self.index >= self.cq.len() { None }
    else {
      let answer = Some(&self.cq[self.index]);
      self.index += 1;
      answer
    }
  }//next
}// Iterator

/// Implementing this trait means we can say `for x in &queue`, which is
/// equivalent to `for x in queue.iter()`.
impl<'lt,T, const C:usize> IntoIterator for &'lt CircularQueue<T,C> {
  type Item = &'lt T;
  type IntoIter = Cqiter<'lt,T,C>;
  fn into_iter(self) -> Self::IntoIter {  // but this self is a &'lt Circ..
    self.iter()
  }
}//IntoIterator

/// function to test circular queue.
pub fn main() {
  let mut cq = CircularQueue::<usize>::new();
  for i in 0..100 {
    cq.push_back(i*2);
    cq.push_front(i*2 + 1);
  }
  cq.mapfun(|x|*x = *x+1);
  for _ in 0..50 {
    cq.pop_front();
    cq.pop_back();
  }
  //cq.mapfun(|x|print!("{} ",&x));
  for x in &cq { print!("{} ",x); }
  println!("\nsize: {}, capacity {}", cq.len(), cq.capacity());
  println!("cq[5] : {}", cq[5]);
  cq[5] = 99999;
  println!("cq[5] : {}", cq[5]);
  let mut sum = 0;
  cq.mapfun(|x| sum += *x);
  println!("sum = {}",sum);

}//main
