//!  **Rust Assignment 2.** Instructions are in the documentation for the [TwowayMap]
//! struct.


use std::rc::Rc;
use std::collections::HashMap;
use std::hash::Hash;

/// For this assignment you are to complete the implementation of a two-way
/// hashmap (bijective hashmap) by implementing a data structure that contains
/// two hashmaps underneath, one in each direction.  The information in the two
/// maps must be consistent and reflect a one-to-one relationship. You must
/// be able to lookup and remove either value using the other.
/// Since the key and value will have to be stored in both maps, you should
/// use Rc.  I've defined the basic struct for you.
pub struct TwowayMap<TA,TB> {
  forward : HashMap<Rc<TA>,Rc<TB>>,
  backward: HashMap<Rc<TB>,Rc<TA>>,
  size : usize,
}

/// Complete the implementation of the bijective hashmap.  Essentially this
/// means completing the `set` function.
impl<TA:Hash+Eq, TB:Hash+Eq> TwowayMap<TA,TB> {
  /// creates a new twowaymap
  pub fn new() -> Self {
    TwowayMap {
      forward: HashMap::new(),
      backward:HashMap::new(),
      size: 0,
    }
  }//new

  /// insert or change the association between a and b.  Note you will have to
  /// remove previous associations for a and b in either map.  For example,
  /// if you map "Monday" to 1 and "Tuesday" to 2, and call .set("Monday",2),
  /// you will have to delete FOUR entries first before inserting the new ones.
  /// Please note, given an `x:Rc<T>`, `Rc::into_inner(x)` will return `Option<T>`,
  /// taking the value out IF there is only one strong reference count for the
  /// Rc.  Also, given a `x:&Rc<T>`, `x.as_ref()` will give you a `&T`.  This
  /// function should return the previously associated values.
  /// Study the functions that have already been defined for you for hints.
  pub fn set(&mut self, a:TA, b:TB) -> (Option<TA>,Option<TB>) {
  
    let rca = Rc::new(a);
    let rcb = Rc::new(b);
    let rcac = Rc::clone(&rca);
    let rcbc = Rc::clone(&rcb);
    // ... what else?
    (None,None)
  }//set

  /// this function has been defined for you. Note that because of
  /// deref coercion, it's ok to look up with a `&TA`, and not
  /// necessarily a `&Rc<TA>`.  However, you will have to apply the
  /// `as_ref()` transformation on the value that's looked up.
  pub fn forward_get(&self, x:&TA) -> Option<&TB> {
    self.forward.get(x).map(|y|y.as_ref())
  }

  /// complete the definition of this function
  pub fn backward_get(&self,x:&TB) -> Option<&TA> {
    None
  }

  /// This function has been defined for you.  Note that calling
  /// `Rc::into_inner` at the end will succeed because the duplicate
  /// has been removed from the backwards map.
  pub fn forward_remove(&mut self, x:&TA) -> Option<TB> {
    let bopt = self.forward.remove(x);
    if let Some(b) = &bopt {
      self.size-=1;
      self.backward.remove(b);
    }
    bopt.and_then(|x|Rc::into_inner(x))
  }

  /// complete the definition of this function
  pub fn backward_remove(&mut self, x:&TB) -> Option<TA> {
    None
  }

  /// this function has been defined for you
  pub fn len(&self) -> usize { self.size }
  
  /// this function has been defined for you
  pub fn iter<'t>(&'t self) -> impl Iterator<Item=(&'t TA, &'t TB)> {
    self.forward.iter().map(|(rca,rcb)|(rca.as_ref(),rcb.as_ref()))
  }
}

/// Run this main to test your program
pub fn main1() {
  let mut map = TwowayMap::new();
  map.set("Monday",1);
  map.set("Tuesday",2);
  map.set("Wednesday",3);
  map.set("Thursday",4);
  map.set("Friday",5);
  map.set("Saturday",6);
  map.set("Sunday",7);
  println!("{:?}", map.set("Sunday",0));
  for x in map.iter() {
    println!("{:?}",x);
  }


  println!("{:?}", map.backward_get(&1));
  println!("{:?}", map.backward_get(&7));
  println!("{:?}", map.backward_remove(&2));


  println!();
  map.set("Friday",6);
  for x in map.iter() {
    println!("{:?}",x);
  }

  map.set("Monday",1);
  println!("size {}", map.len());
}

