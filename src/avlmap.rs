//! ## **CSC 7B/FC Rust Assignment 2**
//!
//! Study the implementation of the [crate::avltree] module.  Then complete
//! the implementation of the [AVLMap] data structure, which has a skeleton
//! in this module.  Some of the functions you will have to define just
//! involve calling the equivalent function on [avltree::AVLSet], but others
//! you will have to re-define from scratch, following the sample code in
//! [crate::avltree], because you will have to separate the key from the value.
//! A "map" as opposed to a "set" contains [key-value pairs](KVPair). The key
//! implements the [std::cmp::Ord] and [std::cmp::Eq] traits, but the value
//! type can be anything.  The tree is ordered and searched by the key.
//!
//! Note: `cargo new` your own crate and copy whatever you need into it:
//! don't try to edit this crate directly.
//!
//! **Additional Requirement:** all your functions (and any structs/enums) must
//! be properly documented using `cargo doc`.

use crate::avltree::Bst::*;
use crate::avltree::*;
use std::fmt;
//use std::cmp::{PartialOrd,PartialEq};

/// A key-value pair:
pub struct KVPair<KT, VT> {
    pub key: KT,
    pub val: VT,
}
impl<KT: PartialEq, VT> PartialEq for KVPair<KT, VT> {
    fn eq(&self, other: &Self) -> bool {
        &self.key == &other.key
    }
}
impl<KT: Eq, VT> Eq for KVPair<KT, VT> {}

impl<KT: PartialOrd + Eq, VT> PartialOrd for KVPair<KT, VT> {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.key.partial_cmp(&other.key)
    }
}
impl<KT: Ord + Eq, VT> Ord for KVPair<KT, VT> {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.key.cmp(&other.key)
    }
}
impl<KT: fmt::Display, VT: fmt::Display> fmt::Display for KVPair<KT, VT> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({} : {})", &self.key, &self.val)
    }
}

/// convenient function to create a [KVPair]
pub fn newpair<K, V>(k: K, v: V) -> KVPair<K, V> {
    KVPair { key: k, val: v }
}

/// Wrapper for an AVL "map" (as opposed to "set").  
/// **Your assignment is to complete the implementation of this class**
/// by modifying or adding the following methods.
/// ```   
///   pub fn insert(&mut self, key:KT, val:VT) -> bool
///   pub fn get(&self, key:&KT) -> Option<&VT>
///   pub fn take(&mut self, key:&KT) -> Option<KVPair<KT,VT>> //aka remove
///   pub fn iter<'lt>(&'lt self) -> InorderIter<'lt,KVPair<KT,VT>>
///   pub fn successor(&self, key:&KT) -> &Bst<KVPair<KT,VT>>
///   pub fn predecessor(&self, key:&KT) -> &Bst<KVPair<KT,VT>>
///   pub fn main()  // demonstrate (this has to be in main.rs)
/// ```
/// Study the similar methods in the [crate::avltree] module.
///
/// **Additional Hints**
///
/// You will have to add some procedures specific to type `Bst<KVPair<KT,VT>>`.
/// Instead of editing the avltree.rs file, you can do this with another 
/// "impl" block:
/// ```
///   impl<KT:Ord,VT> Bst<KVPair<KT,VT>> {
///     // add new procedures here
///   }
/// ```
///
/// Also, it's ok to move an item out of a struct right before the end
/// of its lifetime (about to be deallocated). Just as an example:
/// ```
///   fn f() -> String {
///     let pair = KVPair{key: String::from("abc"), val:123};
///     pair.key // Ok because pair won't be needed again
///   }
/// ```
///
pub struct AVLMap<KT, VT> {
    inner: AVLSet<KVPair<KT, VT>>,
}
impl<KT: Ord + Eq, VT> AVLMap<KT, VT> {
    /// returns size of map: this I'll do for you because it's easy.
    pub fn len(&self) -> usize {
        self.inner.len()
    }

    /// dummy insert can't insert anything (because it's a dummy).
    pub fn insert(&mut self, key: KT, val: VT) -> bool {
        self.inner.add(newpair(key, val))
        //false
    }

    /// Write a procedure to return reference to value associated with
    /// the key, if it exists.  The supplied dummy procedure returns None.
    /// This is harder because to you have to distinguish the key from the
    /// value.
    pub fn get(&self, key: &KT) -> Option<&VT> {
        let mut current = &self.inner.root;
        while let Node(cell) = current {
            let item = &cell.item;
            if key == &item.key {
                return Some(&item.val);
            } else if key < &item.key {
                current = &cell.left;
            } else {
                current = &cell.right;
            }
        }
        None
    } //get

    /// Write a procedure that removes and returns the key-value pair
    /// associated with the key, if it exists.  The supplied function
    /// just returns None
    pub fn take(&mut self, key: &KT) -> Option<KVPair<KT, VT>> {
        None
    } //delete

    // add your code here.

    /// **Optional problem:** complete the above function.  The result of this
    /// function should be that the map will contain the key (note that it's
    /// an owned key, not a reference) and the key is associated with the value
    /// returned by the supplied closure.  The closure is applied to either
    /// the existing key-value pair, or to None.  The function should return
    /// the previous key-value pair, if it exists.  If you define this function
    /// correctly, then the [Self::insert] function can be implemented by
    /// calling `self.and_modify(key, |_|val).is_none()`.
    pub fn and_modify<F>(&mut self, key: KT, modifier: F) -> Option<KVPair<KT, VT>>
    where
        F: Fn(Option<&KVPair<KT, VT>>) -> VT,
    {
        // write this function
        None
    } // treemap.and_modify(key, |vopt|vopt.map(|x|*x+1).unwrap_or(1))
} //avlmap
