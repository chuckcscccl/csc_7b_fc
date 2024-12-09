//! AVL Binary Search Tree in Rust, 2024 version
//!
//! An [AVL tree](https://en.wikipedia.org/wiki/AVL_tree)
//! is a type of balanced binary search tree, similar to a
//! Red-Black tree.  Note that Rust's `std::collections::{BTreeSet,BTreeMap}`
//! are not binary search trees but B-trees, which are generalizations of
//! BSTs, are more cache-friendly, and offer better amortized performance.
//! However, they're very difficult to implement and may not contain all
//! the features we might need.
//!
//! Because only the public items are documented, you should look at the
//! the source code to fully understand the program.
//!

#![allow(dead_code)]
#![allow(non_snake_case)]
#![allow(unused_variables)]
use std::mem;
//use std::collections::BTreeSet;   // for comparison

/// Enum defining an AVL Tree as either an Empty tree or a Node.
/// This avoids using `Option<Node<T>>` for the left/right subtrees.
pub enum Bst<T> {
    Empty,
    Node(Box<Cell<T>>),
}
use Bst::*;

/// Cell with item, left/right subtrees: look at the source code to understand.
/// Note: usually items inside structs are not made public but I've chosen to
/// do so to make your assignment easier.
pub struct Cell<T> {
    pub item: T,
    height: u8, // 2**255 nodes is a pretty big tree
    pub left: Bst<T>,
    pub right: Bst<T>,
}

impl<T: Ord> Cell<T> {
    pub fn set_height(&mut self) -> i16 {
        // also returns height balance
        let (hl, hr) = (self.left.height(), self.right.height());
        self.height = if hl > hr { hl + 1 } else { hr + 1 };
        (hr as i16) - (hl as i16)
    }

    fn LL(&mut self) {
        // left-to-right rotation
        if let Node(lnode) = &mut self.left {
            mem::swap(&mut self.item, &mut lnode.item); // x,y in right place
            mem::swap(&mut lnode.left, &mut lnode.right); //LR now in right place
            mem::swap(&mut lnode.right, &mut self.right); // R in right place
            lnode.set_height();
            // original L now at R
            mem::swap(&mut self.left, &mut self.right); // L now previous R
        }
        self.set_height();
    } // LL rotation

    fn RR(&mut self) {
        if let Node(rnode) = &mut self.right {
            mem::swap(&mut rnode.item, &mut self.item); // x, y swapped
            mem::swap(&mut rnode.left, &mut rnode.right); // RL in place,
            mem::swap(&mut rnode.left, &mut self.left);
            rnode.set_height();
            mem::swap(&mut self.right, &mut self.left);
        }
        self.set_height();
    } // RR

    pub fn balance(&mut self) {
        let hd = self.set_height();
        if hd < -1 {
            // LL or LR
            if let Node(lnode) = &mut self.left {
                let hll = lnode.left.height();
                let hlr = lnode.right.height();
                if hlr > hll {
                    lnode.RR();
                }
                self.LL();
            }
        }
        // LL/LR
        else if hd > 1 {
            // RR or RL
            if let Node(rnode) = &mut self.right {
                let hrl = rnode.left.height();
                let hrr = rnode.right.height();
                if hrl > hrr {
                    rnode.LL();
                }
                self.RR();
            }
        } // right side
    } // balance
} //impl Cell

/// A "default" Bst is an empty tree.
impl<T> Default for Bst<T> {
    fn default() -> Self {
        Empty
    }
}

/// The implementation of AVL trees assumes the type `T` implements a
/// total ordering, not just a partial ordering
impl<T: Ord> Bst<T> {
    /// Creates a new leaf node that takes ownership of `val`
    pub fn new_leaf(val: T) -> Bst<T> {
        Node(Box::new(Cell {
            item: val,
            left: Empty,
            right: Empty,
            height: 1,
        }))
    }

    /// Returns the height (or depth) of the subtree rooted at the self node.
    /// This is an O(1) operation because AVL trees store the height value
    /// at each node.
    pub fn height(&self) -> u8 {
        match self {
            Empty => 0,
            Node(bx) => bx.height,
        }
    } // height

    /// Returns a reference of the left subtree, which is Empty if
    /// it doesn't exist
    pub fn get_left(&self) -> &Bst<T> {
        match self {
            Empty => self,
            Node(bx) => &bx.left,
        }
    }
    /// Returns a reference of the right subtree, which is Empty if
    /// it doesn't exist
    pub fn get_right(&self) -> &Bst<T> {
        match self {
            Empty => self,
            Node(bx) => &bx.right,
        }
    }
    /// Returns an (immutable) reference to the item stored at the root at
    /// this subtree, if it exists
    pub fn get_item(&self) -> Option<&T> {
        match self {
            Empty => None,
            Node(cell) => Some(&cell.item),
        }
    }

    /// Determines if v is found in the subtree rooted at self.
    /// Look at the source code: it doesn't use recursion to maximize
    /// efficiency.
    pub fn search(&self, v: &T) -> bool {
        let mut current = self;
        while let Node(cell) = current {
            if v == &cell.item {
                return true;
            } else if v < &cell.item {
                current = &cell.left;
            } else {
                current = &cell.right;
            }
        }
        false
    } //search

    /// Inserts a new value v into the subtree, avoiding duplicates.
    /// The procedure returns true if insertion was successful (v is not a
    /// duplicate). Look at the source code:
    /// this procedure is **recursive**, but with **zero-overhead** in the
    /// sense that it is not less efficient than a non-recursive version
    /// (which is much more difficult to write).  To maintain a balanced
    /// tree, rotations must be applied after an insertion (or removal)
    /// as we "travel back up to the root."  This may suggest having a
    /// "parent" pointer at each node, which is bad in any language but
    /// especially in Rust, because the parent and left/right pointers
    /// would form **cycles** in memory.  An alternative is to maintain
    /// a stack of references to ancestor nodes as we descend from the root
    /// to the place of insertion.  This is preferable to having parent
    /// pointers because the size of the stack is proportional to only
    /// the log(n) height of the tree.  However, it is also possible to
    /// just use the runtime call-stack for this purpose by using recursion:
    /// as we return from the recursive calls on the left or right subtrees,
    /// we are naturally returning back up to the root. All we have to do
    /// is to re-balance the tree after
    /// each recursive call.  Because a non-recursive procedure must also
    /// create a stack, there is virtually no cost to using recursion.
    pub fn insert(&mut self, v: T) -> bool {
        // returns false if duplicate
        let answer; // must be assigned to later
        match self {
            Empty => {
                answer = true;
                *self = Bst::new_leaf(v);
            }
            Node(cell) => {
                if &v == &cell.item {
                    return false;
                }
                // duplicate not inserted
                else if &v < &cell.item {
                    answer = cell.left.insert(v);
                    if answer {
                        cell.balance();
                    }
                } else {
                    answer = cell.right.insert(v);
                    if answer {
                        cell.balance();
                    }
                }
            }
        } //match
        answer
    } //insert

    /// Delete the value `==` to v, if it exists, returns true if something
    /// was removed.
    pub fn delete(&mut self, v: &T) -> bool {
        let answer;
        match self {
            Empty => {
                return false;
            }
            Node(cell) => {
                if v == &cell.item {
                    // found item to be deleted
                    answer = true;
                    match &cell.left {
                        Empty => {
                            let mut temp = Empty;
                            core::mem::swap(&mut temp, &mut cell.right);
                            *self = temp;
                        }
                        _ => {
                            // delete max node on left subtree (helper function)
                            cell.item = cell.left.delmax();
                        }
                    }
                }
                // found
                else if v < &cell.item {
                    answer = cell.left.delete(v);
                    if answer {
                        cell.balance();
                    }
                } else {
                    answer = cell.right.delete(v);
                    if answer {
                        cell.balance();
                    }
                }
            }
        } //match
        answer
    } //delete

    fn delmax(&mut self) -> T {
        // helper fn for delete, only call on non-empty
        if let Node(cell) = self {
            match &cell.right {
                Empty => {
                    let mut templeft = Empty;
                    core::mem::swap(&mut templeft, &mut cell.left);
                    let mut tempself = Empty;
                    core::mem::swap(&mut tempself, self);
                    *self = templeft;
                    if let Node(oldself) = tempself {
                        return oldself.item;
                    }
                }
                _ => {
                    let answer = cell.right.delmax();
                    cell.balance();
                    return answer;
                }
            } //match
        }
        panic!("This should never happen!"); //only if called on Empty tree
    } //delmax

    /// returns reference to the minimum value in the tree
    pub fn min(&self) -> Option<&T> {
        let mut current = self;
        while let Node(cell) = current {
            if let Empty = &cell.left {
                return Some(&cell.item);
            } else {
                current = &cell.left;
            }
        } //while
        None
    }

    /// returns the Bst rooted at the left-most node
    pub fn min_node(&self) -> &Bst<T> {
        let mut current = self;
        while let Node(cell) = current {
            if let Empty = &cell.left {
                return current;
            } else {
                current = &cell.left;
            }
        } //while
        current
    }

    /// returns refernence to the maximum value in the tree
    pub fn max(&self) -> Option<&T> {
        let mut current = self;
        while let Node(cell) = current {
            if let Empty = &cell.right {
                return Some(&cell.item);
            } else {
                current = &cell.right;
            }
        } //while
        None
    }

    /// returns the Bst rooted at the right-most node
    pub fn max_node(&self) -> &Bst<T> {
        let mut current = self;
        while let Node(cell) = current {
            if let Empty = &cell.right {
                return current;
            } else {
                current = &cell.right;
            }
        } //while
        current
    }

    /// returns successor node to x in tree (could be empty).
    /// the [Self::get_item] procedure can then be called to retrieve the item.
    /// The predecessor procedure should be symmetric.
    pub fn successor(&self, x: &T) -> &Bst<T> {
        let mut ancestor = &Empty;
        let mut current = self;
        while let Node(cell) = current {
            if x < &cell.item {
                ancestor = current;
                current = &cell.left;
            } else if x > &cell.item {
                current = &cell.right; // but ancestor doesn't change
            } else {
                // found x
                if let Empty = &cell.right {
                    return ancestor;
                } else {
                    return cell.right.max_node();
                }
            }
        } //while
        &Empty // the lifetime of this reference is 'static, so ok to return
    } // successor

    /// Preorder traversal with ancestor nodes.  Applies the closure f
    /// to each item of the subtree in preorder.  The "right ancestor"
    /// is the closest ancestor to the right of this subtree and the
    /// "left ancestor" is the closest ancestor to the left of this subtree.
    ///      right ancestor
    ///         /
    ///         \
    ///      left ancestor
    ///           \  
    ///           self
    pub fn map_preorder<'t, F>(
        &'t self,
        right_ancestor: &'t Bst<T>,
        left_ancestor: &'t Bst<T>,
        f: &F,
    ) where
        F: Fn(&T, &Bst<T>, &Bst<T>),
    {
        match self {
            Empty => {}
            Node(cell) => {
                f(&cell.item, right_ancestor, left_ancestor);
                cell.left.map_preorder(self, left_ancestor, f);
                cell.right.map_preorder(right_ancestor, self, f);
            }
        }
    } //map_preorder
} //impl Bst

/// Exterior wrapper class for an AVL Set with
/// an O(1) [AVLSet::len] function.
pub struct AVLSet<T> {
    /// The root is public to make your assignment easier
    pub root: Bst<T>,
    /// The size is public to make your assignment easier
    pub size: usize,
} //AVLSet

impl<T: Ord> AVLSet<T> {
    /// creates an empty set
    pub fn new() -> Self {
        AVLSet {
            root: Empty,
            size: 0,
        }
    }
    /// returns cardinality of the finite set
    pub fn len(&self) -> usize {
        self.size
    }
    /// returns the height of the AVL tree
    pub fn height(&self) -> usize {
        self.root.height() as usize
    }
    /// inserts value x into set, avoiding duplicates, returns false if
    /// x already exists in set
    pub fn add(&mut self, x: T) -> bool {
        let answer = self.root.insert(x);
        if answer {
            self.size += 1;
        }
        answer
    }
    /// determines if x is inside the set
    pub fn contains(&self, x: &T) -> bool {
        self.root.search(x)
    }
    /// removes x from set, returns false if x was not in set
    pub fn remove(&mut self, x: &T) -> bool {
        let answer = self.root.delete(x);
        if answer {
            self.size -= 1;
        }
        answer
    } //remove

    /// returns an in-order iterator over the set
    pub fn iter<'t>(&'t self) -> InorderIter<'t, T> {
        InorderIter { cells: Vec::new() }
    }
} // impl AVLSet

/////////////// Iterators /////////////////

/// In-order Iterator structure
pub struct InorderIter<'lt, T> {
    cells: Vec<&'lt Cell<T>>, // stack of cell references
}
impl<'lt, T> Iterator for InorderIter<'lt, T> {
    type Item = &'lt T;
    fn next(&mut self) -> Option<Self::Item> {
        let nextcell = self.cells.pop()?;
        let mut current = &nextcell.right;
        while let Node(lcell) = current {
            self.cells.push(&lcell);
            current = &lcell.left;
        } //while
        Some(&nextcell.item)
    } //next
}
impl<T> Bst<T> {
    /// Creates an in-order iterator
    pub fn iter<'lt>(&'lt self) -> InorderIter<'lt, T> {
        let mut cells = vec![];
        let mut current = self;
        while let Node(lcell) = current {
            cells.push(&**lcell);
            current = &lcell.left;
        }
        InorderIter { cells }
    } //iter
}
impl<'lt, T> IntoIterator for &'lt Bst<T> {
    type Item = &'lt T;
    type IntoIter = InorderIter<'lt, T>;
    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}

// for testing
fn main1() {
    let mut tree = Bst::<i32>::new_leaf(5);
    for x in [2, 7, 8, 1, 2, 4, 3, 9] {
        tree.insert(x);
    }

    println!("search 7: {}", tree.search(&7));
    println!("search 11: {}", tree.search(&11));
    tree.delete(&7);
    tree.delete(&4);
    println!("search 7: {}", tree.search(&7));
    for x in tree.iter() {
        println!("iter {}", x);
    }

    /*   // attempt to create loop in tree:
    let mut root = Box::new(Bst::<i32>::new_leaf(8));
    for x in [9,3,7,10,13,11,2,5,4] { root.insert(x); }
    //root.left = Some(root);  // compiler error.  the world is safe.
    */

    //bigtest(10000000);
} // main

fn bigtest(n: i32) {
    let mut tree = AVLSet::<i32>::new();
    for x in 0..n {
        tree.add(x);
    }
    //println!("after insert: size {}, height {}",tree.len(),tree.height());
    for x in 0..(n / 2) {
        tree.remove(&x);
    }
    println!(
        "after delete: size {}, height {}",
        tree.len(),
        tree.height()
    );
}
