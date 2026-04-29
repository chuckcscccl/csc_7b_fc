//!  **CSC 123/252 Assignment**
//!
//!  This module contains the skeleton of a bijective hashmap that you should
//!  implement fully.  It is based on the implementation of my version of
//!  one-way hashmaps found in [crate::hmap].  Consult the documentation
//!  and source code of [crate::hmap::Hmap] to understand it before proceeding.
//!  The idea is to use two Hmaps underneath, each mapping a key to the index
//!  in the other map's underlying vector where the corresponding value is
//!  stored.  This way a bijective hashmap can be constructed without
//!  resorting to Copy, Clone, Rc, or anything unsafe.  Click on the
//!  [Bimap] structure link to see the functions you need to implement.
//!
//!  As a challenge, try to also implement an iterator for your structure.
//!  Try to use the iterator for [Hmap].

#![allow(dead_code, unused_imports, unused_variables)]

use crate::hmap::*;
use std::hash::{BuildHasher,Hash,Hasher};

//////////////Bimap
pub struct Bimap<TA,TB> {
  pub forward : Hmap<TA,usize>,
  pub backward: Hmap<TB,usize>,
}
impl<TA:Hash+Eq, TB:Hash+Eq> Bimap<TA,TB> {

  /// Constructs a new Bimap with default capacity 16.  This function has
  /// been written for you.
  pub fn new() -> Self {
    Bimap {
      forward : Hmap::new(),
      backward: Hmap::new(),
    }
  }

  /// Constructs a new Bimap with requested capacity.  The actual capacity
  /// will be the closest power of two that's greater than or equal to the
  /// requested capacity.  This function has been written for you.
  pub fn with_capacity(cap:usize) -> Self {
    Bimap {
      forward: Hmap::with_capacity(cap),
      backward: Hmap::with_capacity(cap),
    }
  }

  /// Gets the value associated with the key in the forward direction, if
  /// it exists.  This function has been written for you.
  pub fn get_forward(&self, key:&TA) -> Option<&TB> {
    match self.forward.get(key) {
      Some(vi) => self.backward.table[*vi].as_ref().map(|(v,_)|v),
      None => None,
    }
  }

  /// Complete the implementation of this function
  pub fn get_backward(&self, key:&TB) -> Option<&TA> {
    None
  }

  /// Removes the value associated with the key in the forward direction,
  /// returning the existing key-value pair, if it exists.  This function
  /// has been written for you.
  pub fn remove_forward(&mut self, key:&TA) -> Option<(TA,TB)> {
    let rf = self.forward.remove(&key);
    rf.and_then(|(k,vi)|{
      let mut temp = None;
      core::mem::swap(&mut temp, &mut self.backward.table[vi]);
      self.backward.count -= 1;
      temp.map(|(v,ki)|(k,v))
    })
  }

  /// Complete the implementation of this function
  pub fn remove_backward(&mut self, key:&TB) -> Option<(TA,TB)> {
    None
  }  

  /// Complete the implementation of this function.  The general approach is
  /// the following.  First, call [Bimap::remove_forward] and [Bimap::remove_backward] to
  /// clear existing values.  Then call [Hmap::find_new_slot]
  /// to find appropriate places to place the key and values in the forward
  /// and backward maps.  Then place in the forward map `Some((key,vi))`
  /// where vi is the index in the backward map where the value
  /// is found.  And place in the backwards map `Some((val,ki))` where ki
  /// is the index in the forward map where the key is found.
  /// Besure to call resize if necessary when adding manually to the forward
  /// and backwards maps, as well as increase their `count`.
  ///
  /// Note that this function returns a pair of options because both the
  /// key and value could've had prior associations that must be deleted.
  pub fn set(&mut self, key:TA, val:TB) -> (Option<(TA,TB)>,Option<(TA,TB)>) {
    (None,None)
  }

  /// Returns the number of key-value pairs in the bijective map.  This function
  /// has been written for you, unless you decide to change it.
  pub fn len(&self) -> usize { self.forward.len() }

}// impl

fn test() {
  let mut days = Bimap::new();
  days.set("Monday",1);
  days.set("Tuesday",2);
  days.set("Wednesday",3);
  days.set("Thursday",4);
  days.set("Friday",5);
  days.set("Saturday",6);
  days.set("Sunday",7);
  println!("tuesday to 4: {:?}", days.set("Tuesday",4));
  println!("{:?}",days.get_forward(&"Sunday"));
  println!("{:?}",days.get_backward(&4));
  println!("{:?}",days.get_backward(&7));
  println!("size {}", days.len());
  /*
  for (k,v) in days.iter() {
    println!("{} : {}", k, v);
  }
  */
}

