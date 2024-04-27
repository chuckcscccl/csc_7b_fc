//! Skeleton module for the bijective hashmap assignment.
//!
//! Be sure to read the assignment description in the 
//! **[C++ version](https://github.com/chuckcscccl/csc_7b_fc/blob/main/src/bijectivemap.cpp)** of this 
//! program that you're supposed to emulate.
//! 
//! This module currently compiles and provides you with some helpful code,
//! but it contains some dummy implementations that you must implement.
//! properly (see documentation for each [BijectiveMap] method).
//! Place the following in the main.rs of your crate.
//! ```
//!    mod bijectivemap;
//!    use bijectivemap::*;
//! ```
//!
//! When you have completed the implementation, emulate the main in the
//! C++ program to test it.  It can begin with ...
//!```
//!    let mut daynum:BijectiveMap<&'static str,u8> = BijectiveMap::new();
//!    let days = ["Monday","Tuesday","Wednesday","Thursday","Friday","Saturday","Sunday"];
//!    for i in 0..days.len() { daynum.set(days[i],(i+1) as u8); }
//!    daynum.get_by_key(&"Wednesday").map(|d|{
//!      println!("Wednesday is day {}",d);
//!    });
//!```
//!
#![allow(unused_mut)]
use std::collections::{HashMap, hash_map::RandomState};
use std::hash::{BuildHasher, Hash, Hasher};

///The `HashStealer` struct and methods allows you to "steal" the hash functions 
///from Rust, and base_hash can be called on any key that implements Hash+Eq.
///Hashing requires equality and must be consistent with it: if x==y then
///hash(x)==hash(y).

pub struct HashStealer {
  state: RandomState,
}
impl HashStealer {
  /// creates a new hash stealer. Here, `Self` refers to the type being
  /// "impl"ed while `self` refers to the current instance of `Self`.
  pub fn new() -> Self {
    HashStealer { state: RandomState::new(), }
  }//new
  
  /// do what's needed to return a hash value on any hashable key
  pub fn base_hash<K:Hash+Eq>(&mut self, key: &K) -> usize {
    let mut builder = self.state.build_hasher();
    key.hash(&mut builder);
    builder.finish() as usize
  }//base_hash
}//impl HashStealer

/// This is can be your bijective map struct
pub struct BijectiveMap<KT, VT, const CAP:usize = 16> {
  keys: HashMap<usize,Vec<(KT,usize,usize)>>,
  vals: HashMap<usize,Vec<(VT,usize,usize)>>,
  size: usize,
  hasher: HashStealer,
}// struct BijectiveMap

// some more code to get you started ...
// Under no circumstances can KT, VT impl any trait other than Hash and Eq:
impl<KT:Hash+Eq, VT:Hash+Eq, const CAP:usize> BijectiveMap<KT,VT,CAP> {

  // non-public internal function to use HashStealer
  fn hash<K:Hash+Eq>(&mut self, key: &K) -> usize {
    self.hasher.base_hash(key)
  }//hash

   /// creates a new BijectiveMap with default initial capacity:
   pub fn new() -> Self  {  // Self refers to this TYPE, BijectiveMap
    BijectiveMap {
      keys: HashMap::with_capacity(CAP), // just initial capacity
      vals : HashMap::with_capacity(CAP), // can expand if needed
      size: 0,
      hasher : HashStealer::new(),
    }
  }//new

   /// Returns a immutable reference to the value corresponding to
   /// the given key, if it exists.  Note that there can't be a 
   /// Option<&mut T> version of this method because changing the
   /// value will require adjusting other associations in the map.
   pub fn get_by_key(&mut self, key:&KT) -> Option<&VT> {
    let hk = self.hash(key);
    if !self.keys.contains_key(&hk) {return None;}
    for (k,vr,vc) in self.keys[&hk].iter() {
        if k==key {
          return Some(&self.vals[vr][*vc].0);  //.0 is first in tuple
        }
    }
    None
  }//get

} // impl

/// place all your code in another impl block, with the following functions
/// fully implemented
impl<KT:Hash+Eq, VT:Hash+Eq, const CAP:usize> BijectiveMap<KT,VT,CAP> {

   /// Returns number of key-value pairs in this map.
   /// This one's too easy so I just did it for you again.  I will give
   /// away more stuff if you come to me for help ...
   pub fn len(&self) -> usize { self.size } 

   /// Returns an immutable reference to the key associated with the
   /// given value, if it exists. The dummy provided here just returns
   /// `None` and you need to rewrite it appropriately.
   pub fn get_by_val(&mut self, val:&VT) -> Option<&KT> 
   { None }

  /// Removes key-value pair from map by searching for it by key.
  /// Returns the key and value if found.  Note that this function
  /// is "take" and not "get" because it **moves** the actual key and value
  /// out of the map.  It doesn't return references.  Sometimes you 
  /// may need to convert an `Option<T>` to an `Option<&T>`.  To do
  /// that, given such an `Option<T>` `opt`, use the expression `opt.as_ref()`.
  ///
  /// The dummy implemenetation provided here just returns `None`
  pub fn take_by_key(&mut self, key:&KT) -> Option<(KT,VT)>
  { None }

  /// Dummies always return `None`.  Are you a dummy?
  pub fn take_by_val(&mut self, val:&VT) -> Option<(KT,VT)>
  { None }

  /// Change or add key-val pair, return a pair to represent information
  /// that was deleted.  Note that the returned key,value, if they 
  /// exist, may indicate a different key or value that existed in the
  /// map before the change (see "daynum" example that you should
  /// replicate in main).  And don't just be a dummy like the current
  /// state of this implementation.
  pub fn set(&mut self, mut key:KT, mut val:VT) -> Option<(KT,VT)>
  { None }

} // can have multiple impl blocks
