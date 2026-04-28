//! This module contains the implementation of a closed hashmap from scratch.
//! Because the interior of the implementation is exposed, one can use it to
//! implement a biject hashmap without resorting to Copy, Clone, Rc or anything
//! unsafe.

#![allow(dead_code, unused_imports, unused_variables)]

use std::collections::hash_map::RandomState;
use std::hash::{BuildHasher, Hash,Hasher};
use std::ops::{Index,IndexMut};

/// Structure of a HashMap, built from scratch, but which uses
/// Rust's built-in implmenetations of the Hash trait.
/// This is a closed hash table with a linear probing rehash function.
/// The table is a vector of Option<key-value-pair>.  The vector
/// maxhashes tracks the maximum number of hashes and rehashes required
/// for a given hash slot.  keylocs tracks locations of possible keys
/// in the vector, which is important for quick iteration over the structure.
/// The size/capacity of the vector is always set to a power of 2: this
/// means instead of % capacity we can calculate & mask, where mask is always
/// capacity-1;  The hashstate is necessary for writing the hash function.
pub struct Hmap<KT,VT> {
  pub table : Vec<Option<(KT,VT)>>,
  pub count : usize,
  pub maxhashes: Vec<usize>,
  pub keylocs : Vec<usize>, // records where keys are located.
  pub mask : usize,  // always = capacity-1 (power of 2 - 1)
  pub hashstate : RandomState,
}

impl<KT:Hash+Eq,VT> Hmap<KT,VT> {
  fn make(cap : usize) -> Self {  // internal function
    let mut tab = Vec::new();
    tab.resize_with(cap,||None);
    let mut mh = Vec::new();
    mh.resize(cap,0);
    Hmap {
      table : tab,
      count : 0,
      maxhashes : mh,
      keylocs: Vec::with_capacity(cap),
      mask : cap-1,
      hashstate: RandomState::new(),
    }
  }// with_capacity

  /// Creates new Hmap with default capacity 16
  pub fn new() -> Self {
    Self::make(16)
  }

  /// Creates new Hmap with requested capacity.  The
  /// actual capacity will be the nearest power of two that's
  /// greater than or equal to the requested capacity.
  pub fn with_capacity(requested_cap:usize) -> Self {
    let mut cap = 1;
    while cap < requested_cap { cap *= 2; }
    Self::make(cap)
  }

  /// returns the number of key-value pairs inside the map
  pub fn len(&self) -> usize { self.count }
  /// returns the current capacity of the map
  pub fn current_capacity(&self) -> usize { self.mask+1 }

  /// retrieves a borrow of the value associated with the given key,
  /// if it exists.  Look at the source code to understand how it's defined.
  /// Given an opt:Option<T>, opt.as_ref() will return an Option<&T>.
  pub fn get(&self, key:&KT) -> Option<&VT> {
    match self.find_slot(key) {
      Some(h) => self.table[h].as_ref().map(|(k,v)|v),
      None => None,
    }
  }

  /// retrieves a mutable borrow of the value associated with the given key,
  /// if it exists.  Look at the source code.  Given a mutable opt:Option<T>,
  /// opt.as_mut() will return an Option<&mut T>.
  pub fn get_mut(&mut self, key:&KT) -> Option<&mut VT> {
    match self.find_slot(key) {
      Some(h) => self.table[h].as_mut().map(|(k,v)|v),
      None => None,
    }
  }

  /// add or change the value associated with the key, returns the
  /// previous key and value, if it existed.  Look at the source code.
  /// Note that core::mem::swap is used to assign to the vector while
  /// taking the previous value out of the vector.
  pub fn set(&mut self, key:KT, val:VT) -> Option<(KT,VT)> {
    if self.count*100 >= (self.mask+1)*75 { self.resize(true); }
    match self.find_new_slot(&key) {
      (h,true) => {
          let mut pair = Some((key,val));
          core::mem::swap(&mut pair, &mut self.table[h]);
          pair
        },
      (h,false) => {
          self.count += 1;
          let pair = Some((key,val));
          self.table[h] = pair;
          None
        },
    }//match
  }

  /// removes the key-value pair given the key, returns the pair if it
  /// exists.  Note that core::mem::swap is used to take the pair out
  /// of the vector.
  pub fn remove(&mut self, key:&KT) -> Option<(KT,VT)> {
    match self.find_slot(key) {
      Some(h) => {
        let mut temp = None;
        core::mem::swap(&mut temp, &mut self.table[h]);
        self.count -= 1;
        temp
      },
      None => { None },
    }
  }

  /// Internal function called by resize, assumes that key-value pair
  /// is new and that table has enough capacity.
  pub fn add(&mut self, key:KT, val:VT) {  // called internally
    let h0 = self.hash(&key);
    let mut h = h0;
    let mut hashes = 1;
    while let Some(_) = &self.table[h] {
      h = (h+1)&self.mask;
      hashes += 1;
    }
    if hashes > self.maxhashes[h0] { self.maxhashes[h0] = hashes; }
    self.keylocs.push(h);
    self.count += 1;
    self.table[h] = Some((key,val));
  }

  /// Resizes the hashmap by doubling or halfing the capacity.
  pub fn resize(&mut self, upsize:bool) -> bool {
    let newcap = if upsize {(self.mask+1)*2} else {(self.mask+1)/2};
    if self.count > newcap { return false; }
    let mut newmap = Self::make(newcap);
    for i in &self.keylocs {
      if let Some(_) = &self.table[*i] {
        let mut temp = None;
        core::mem::swap(&mut temp, &mut self.table[*i]);
        temp.map(|(k,v)|{newmap.add(k,v)});
      }
    }
    core::mem::swap(self, &mut newmap);
    true
  }

  /// hash function steals the Hash trait implementations of Rust
  pub fn hash(&self, key:&KT) -> usize {
    let mut bs = self.hashstate.build_hasher();
    key.hash(&mut bs);
    (bs.finish() as usize) & self.mask
  }

  /// This function finds the location where a key is found, or
  /// where a new key-value pair can be inserted.  It returns
  /// an index, paired with a boolean indicating whether the key
  /// was found at that index.
  pub fn find_new_slot(&mut self, key:&KT) -> (usize,bool) {
    let h0 = self.hash(key);
    let mut h = h0;
    let mut hashes = 1;
    let mut reuse = None;
    loop {
      match &self.table[h] {
        Some((k,_)) if key==k => { return (h,true); },
        Some(_) => {
          h = (h+1)&self.mask;
          hashes += 1;
        },
        None if hashes <= self.maxhashes[h0] => {
          if let None = reuse { reuse = Some(h); }
          h = (h+1)&self.mask;
          hashes +=1;
        },
        _ => { break; }  // end loop
      }
    }//loop
    if hashes > self.maxhashes[h0] { self.maxhashes[h0] = hashes; }
    if let Some(r) = reuse { (r,false) }
    else {
      self.keylocs.push(h);
      (h,false)
    }
  }// find_new_slot

  /// This function finds the index of the internal vector where a
  /// key is located, if it exists.
  pub fn find_slot(&self, key:&KT) -> Option<usize> {
    let h0 = self.hash(key);
    let mut h = h0;
    let mut hashes = 1;
    loop {
      match &self.table[h] {
        Some((k,_)) if key==k => { return Some(h); },
        _ if hashes < self.maxhashes[h0] => {
          h = (h+1)&self.mask;
          hashes += 1;
        },
        _ => { break; }
      }
    }//loop
    None
  }// find_slot

  /// Creates iterator over all key-value pairs
  pub fn iter<'t>(&'t self) -> HmapIter<'t,KT,VT> {
    HmapIter {
      hmap : self,
      i : 0,
    }
  }
} // main impl


/// Structure for implementing Iterator trait
pub struct HmapIter<'t,KT,VT> {
  hmap : &'t Hmap<KT,VT>,
  i : usize,
}
impl<'t,KT,VT> Iterator for HmapIter<'t,KT,VT> {
  type Item = (&'t KT,&'t VT);
  fn next(&mut self) -> Option<Self::Item> {
    while self.i < self.hmap.keylocs.len() {
      let ai = self.hmap.keylocs[self.i];
      self.i += 1;
      if let None = &self.hmap.table[ai] { continue; }
      else { return self.hmap.table[ai].as_ref().map(|p|(&p.0, &p.1)); }
    }
    None
  }
}

impl<KT:Hash+Eq,VT> Index<&KT> for Hmap<KT,VT> {
  type Output = VT;
  /// Provides ability to use syntax such as `map[&key]`.  Operation
  /// will panic if key does not exist
  fn index(&self, key:&KT) -> &Self::Output {
    self.get(key).unwrap()
  }
}

impl<KT:Hash+Eq,VT> Index<KT> for Hmap<KT,VT> {
  type Output = VT;
  /// Provides ability to use syntax such as `map[key]`. Operation will
  /// panic if key does not exist.
  fn index(&self, key:KT) -> &Self::Output {
    self.get(&key).unwrap()
  }
}

impl<KT:Hash+Eq,VT:Default> IndexMut<KT> for Hmap<KT,VT> {
  /// Provides ability to use syntax such as `map[key] = val`.
  /// This function will insert a new value into the table if necessary
  /// and assumes that the value type VT implements the Default trait.
  fn index_mut(&mut self, key:KT) -> &mut Self::Output {
    if self.count*100 >= (self.mask+1)*75 { self.resize(true); }  
    let (h,found) = self.find_new_slot(&key);
    if !found {
      self.table[h] = Some((key,VT::default()));
      self.count += 1;
    }
    self.table[h].as_mut().map(|(_,v)|v).unwrap()    
  }
}

