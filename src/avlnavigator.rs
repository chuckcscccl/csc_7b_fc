//! ## AVL Tree Navigator Module.
//! A "tree navigator" is similar in concept to an interator. It works by
//! keeping a "current" node and a stack of "ancestors" along
//! with whether current is on the left or right of the ancestor.
//! With this information we can navigate to any part of tree.
//! Only immutable operations on the tree are permitted with a navigator.
//! Example:
//! ```
//!   fn f<'lt, T:Ord>(tree:&'lt Bst<T>, key:&T) -> Option<&'lt T> {
//!     let mut navigator = AVLNavigator::start(tree);
//!     navigator.seek(key);
//!     navigator.goto_predecessor();
//!     navigator.current_item()
//!   }
//! ```
//!
use crate::avltree::Bst::*;
use crate::avltree::*;
use crate::avlmap::KVPair;

/// Structure for a Tree Navigator
#[derive(Clone)]
pub struct AVLNavigator<'lt,T> {
  ancestors : Vec<(&'lt Bst<T>,bool)>,  // bool true = left, false = right;
  current : &'lt Bst<T>,
}
impl<'lt,T:Ord> AVLNavigator<'lt,T> {

  /// creates a new navigator with given tree, which could be empty,
  /// the current "pointer".
  pub fn start(tree: &'lt Bst<T>) -> Self {
    AVLNavigator {ancestors:Vec::new(), current:tree}
  }

  /// returns the current node (or Empty)
  pub fn get_current(&self) -> &'lt Bst<T> { 
     self.current
  }
  /// alias for [Self::get_current]
  pub fn now(&self) -> &'lt Bst<T> { 
     self.current
  }  

  /// Navigate to the left child.  If there is no left child,
  /// the navigator stays at the same point.  Returns true on
  /// successful navigation
  pub fn go_left(&mut self) -> bool {
     let mut answer = false;
     if let Node(cell) = self.current {
       let left = &cell.left;
       if let Node(_) = left {
         answer = true;
         self.ancestors.push((self.current,true));
         self.current = left;
       }
     }
     answer
  }

  /// Navigate to the right child.  If there is no right child,
  /// the navigator stays at the same point and the function returns false.
  pub fn go_right(&mut self) -> bool {
     let mut answer = false;     
     if let Node(cell) = self.current {
       let right = &cell.right;
       if let Node(_) = right {
         answer = true;
         self.ancestors.push((self.current,false));
         self.current = right;
       }       
     }
     answer
  }

  /// Attempts to navigate to the parent node.  If there is no parent,
  /// however, the navigator stays at the same point. Returns true on
  /// successful navigation
  pub fn go_up(&mut self) -> bool {
    let mut answer = false;
    let parent = self.ancestors.pop();
    parent.map(|(p,_)|{self.current=p; answer=true;});
    answer
  }

  /// alias for [Self::go_up]
  pub fn goto_parent(&mut self) -> bool { self.go_up() } //alias

  /// Navigate to the successor node, or stay at the same node if the successor
  /// doesn't exist. The successor is either the leftmost child of the
  /// right subtree, or, if the right subtree does not exist, the
  /// closest ancestor that the current node is to the left of.
  pub fn goto_successor(&mut self) -> bool {
    let mut answer = false;
    if let Node(cell) = self.current {
      if let Node(_) = &cell.right {
        answer = true;
        self.go_right();
        self.goto_leftmost();
      }// successor on right subtree
      else {
        let mut i = self.ancestors.len();
        while i>0 {
          let ((ancestor,dir)) = self.ancestors[i-1];
          if dir {
            answer = true;
            self.current = ancestor;
            self.ancestors.truncate(i-1);
            break;
          }          
          i -= 1;
        }
      } // successor is closest "left" ancestor
    }
    answer
  }//goto_successor

  /// Navigate to the predecessor node, or stay at the same node if the 
  /// predecessor doesn't exist and the function returns false.
  pub fn goto_predecessor(&mut self) -> bool {
    let mut answer = false;
    if let Node(cell) = self.current {
      if let Node(_) = &cell.left {
        answer = true;
        self.go_left();
        self.goto_rightmost();
      }
      else {
        let mut i = self.ancestors.len();
        while i>0 {
          let ((ancestor,dir)) = self.ancestors[i-1];
          if !dir {
            answer = true;
            self.current = ancestor;
            self.ancestors.truncate(i-1);
            break;
          }          
          i -= 1;
        }//while
      }
    }
    answer
  }//goto_successor

  /// Navigate back to the starting node of the navigator, returns true
  /// on successful navigation.
  pub fn goto_root(&mut self) -> bool {
    if self.ancestors.len()>0 {
      self.current = self.ancestors[0].0;
      self.ancestors.clear();
      true
    }
    else {false}
  }

  /// returns the value inside the current node, if it exists
  pub fn current_item(&self) -> Option<&'lt T> {
    self.get_current().get_item()
  }

  /// Navigate to the node sharing the same parent as the current node,
  /// returning false if there is no sibling.
  pub fn goto_sibling(&mut self) -> bool {
    let mut answer = false;
    if self.ancestors.len()==0 { return false; }
    let (parent,dir) = self.ancestors[self.ancestors.len()-1];
    let sibling =  if dir { parent.get_right() } else { parent.get_left() };
    if let Node(_) = sibling {
      answer = true;
      self.current = sibling;      
    }
    answer
  }

  /// Navigate to sibling of parent
  pub fn goto_aunt(&mut self) -> bool {
    if (self.goto_parent()) {
      self.goto_sibling()
    }
    else {false}
  }
  /// Navigate to sibling of parent
  pub fn goto_uncle(&mut self) -> bool { self.goto_aunt() }

  /// go to right most node (containing maximum vaule) of current subtree
  pub fn goto_rightmost(&mut self) -> bool {
     if let Empty = self.current { return false; }
     while let Node(_) = self.current.get_right() {
       self.go_right();
     }
     true
  }

  /// go to the node containing the minimum value (leftmost node)
  pub fn goto_leftmost(&mut self) -> bool {
     if let Empty = self.current { return false; }     
     while let Node(_) = self.current.get_left() {
       self.go_left();
     }
     true
  }

  /// Search for node containing the given key starting from the current node.
  /// Returns true on success. If the key is not found, the navigator is
  /// restored to its previous state.
  pub fn seek(&mut self, key:&T) -> bool {
    let mut answer = false;
    let savelen = self.ancestors.len();
    let savecurrent = self.current;
    while let Node(cell) = self.current {
      if key == &cell.item {
         answer = true;
         break;
      }
      else if key < &cell.item { //go left
        self.ancestors.push((self.current,true));
        self.current = &cell.left
      }
      else {
        self.ancestors.push((self.current,false));      
        self.current = &cell.right;
      }
    }//while
    if !answer { // restore navigator
      self.ancestors.truncate(savelen);
      self.current = savecurrent;
    }
    answer
  }//goto_item

}// impl AVLNavigator

impl<'lt,T:Ord> Bst<T> {
  /// returns a [AVLNavigator] starting at the current tree
  pub fn new_navigator(&'lt self) -> AVLNavigator<'lt,T> {
    AVLNavigator::start(self)
  }
}

impl<'lt,T:Ord> AVLSet<T> {
  /// returns a [AVLNavigator] starting at the root
  pub fn get_navigator(&'lt self) -> AVLNavigator<'lt,T> {
    AVLNavigator::start(&self.root)
  }
}

impl<'lt,KT:Ord,VT> AVLNavigator<'lt,KVPair<KT,VT>> {
pub fn seek_key(&mut self, key:&KT) -> bool {
    let mut answer = false;
    let savelen = self.ancestors.len();
    let savecurrent = self.current;
    while let Node(cell) = self.current {
      if key == &cell.item.key {
         answer = true;
         break;
      }
      else if key < &cell.item.key { //go left
        self.ancestors.push((self.current,true));
        self.current = &cell.left
      }
      else {
        self.ancestors.push((self.current,false));      
        self.current = &cell.right;
      }
    }//while
    if !answer { // restore navigator
      self.ancestors.truncate(savelen);
      self.current = savecurrent;
    }
    answer
  }//seek
}

// sample function that uses navigator
fn f<'lt, T:Ord>(tree:&'lt Bst<T>, key:&T) -> Option<&'lt T> {
  let mut navigator = AVLNavigator::start(tree);
  navigator.seek(key);
  navigator.goto_predecessor();
  navigator.current_item()
}
