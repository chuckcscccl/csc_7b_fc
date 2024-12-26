// Red-black trees

struct Node<T> {
  item: T,
  left: usize,
  right: usize,   // use usize::Max to mean "None"
  red: bool,      // true if red, else black
}

pub const NIL:usize = usize::MAX;  // empty tree

pub struct RedBlackTree<T> {
  nodes: Vec<Option<Node<T>>>,
  freelist: Vec<usize>,
  size : usize,
  root : usize,  // usize::Max (NIL) means none
}

//// Basic BST implementations

impl<T> Node<T> {
  pub fn new_leaf(i:T) -> Self {
     Node { item:i, left:NIL, right:NIL, red:false, }
  }
  pub fn is_black(&self) -> bool { !self.red }
}

impl<T:Ord> RedBlackTree<T> {
  pub fn with_capacity(cap:usize) -> Self {
    RedBlackTree {
      nodes: Vec::with_capacity(cap),
      freelist: Vec::new(),
      size : 0,
      root : NIL,
    }
  }//with_capacity

  fn cmp(x:&T, opt:&Option<Node<T>>) -> i8 {
    match opt.as_ref() {
      Some(Node{item,left,right,red}) if x==item => 0,
      Some(Node{item,left,right,red}) if x<item =>-1,
      Some(Node{item,left,right,red}) if x>item => 1,
      _ => -2,
    }
  }

  pub fn search(&mut self, x:&T) -> bool {
    let mut current = self.root;
    while current!=NIL {
      match &self.nodes[current] {
        Some(Node{item,..}) if x==item => {return true;},
	Some(Node{item,left,..}) if x<item => { current = *left; },
	Some(Node{item,left,right,..}) if x>item => { current = *right; },
	_ => { break; },
      }//match
    }//while
    false
  }//search

  // insertion
  pub fn insert(&mut self, x:T) -> bool {
    let mut ancestors = vec![];  // stack of parent "pointers" (usize,left)
    let mut current = self.root;
    while current != NIL { // immutable loop to find point of insertion
      match &self.nodes[current] {
	Some(Node{item,left,..}) if &x<item => {
	  ancestors.push((current,true));
	  current = *left;
	},
	Some(Node{item,left,right,..}) if &x>item => {
	  ancestors.push((current,false));
	  current = *right;
	},
	_ => { return false; },        // no duplicates
      }//match
    }//while
    let newnode = Node::new_leaf(x);
    let index;
    if let Some(idx) = self.freelist.pop() {
      index = idx;
      self.nodes[index] = Some(newnode);
    }
    else {
      index = self.nodes.len();
      self.nodes.push(Some(newnode));
    }
    self.size+=1;
    match ancestors.pop() {
      Some((parent, true)) => {
	self.nodes[parent].as_mut().map(|n|n.left = index);
      },
      Some((parent, false)) => {
	self.nodes[parent].as_mut().map(|n|n.right = index);
      },      
      None => {  // insertion is at root
        self.root = index;
      },
    }//match
    true
  }//insert

  // removal
  pub fn remove(&mut self, x:&T) -> Option<T> {
    let mut ancestors = vec![];  // stack of parent "pointers" (usize,left)
    let mut current = self.root;
    while current != NIL { // immutable loop to find point of insertion
      match &self.nodes[current] {
	Some(Node{item,left,..}) if x<item => {
	  ancestors.push((current,true));
	  current = *left;
	},
	Some(Node{item,left,right,..}) if x>item => {
	  ancestors.push((current,false));
	  current = *right;
	},
	Some(Node{item,..}) if x==item => { // found
	  break;
        },	
	_ => { return None; },        // no duplicates
      }//match
    }//while
    if current==NIL { return None; }
    // check if left subtree exists
    let current_left = self.nodes[current].as_ref()?.left;
    if current_left != NIL {
      // replace current node with max node on left (predecessor)
      let mut deleted = self.delmax(self.nodes[current].as_ref()?.left);
      std::mem::swap(&mut deleted,&mut self.nodes[current].as_mut()?.item);
      Some(deleted)
    } 
    else {  // no left subtree, change parent node to point to right 
      match ancestors.pop() {
        Some((parent,true)) => {
	  let current_right = self.nodes[current].as_ref()?.right;
	  self.nodes[parent].as_mut()?.left = current_right;
	},
        Some((parent,false)) => {
	  let current_right = self.nodes[current].as_ref()?.right;
	  self.nodes[parent].as_mut()?.right = current_right;
	},	
	None => {
	  self.root = self.nodes[current].as_ref()?.right;
	}
      }//match
      self.size -= 1;
      self.freelist.push(current);
      let mut answer = None;
      std::mem::swap(&mut answer, &mut self.nodes[current]);
      answer.map(|n|n.item)
    }
  }// remove  

  // remove helper - assume not called on root, nil:
  fn delmax(&mut self, mut current:usize) -> T {
    let mut ancestors = vec![];
    while current!=NIL {
      ancestors.push(current);
      self.nodes[current].as_ref().map(|n|current = n.right);
    }
    let last = ancestors[ancestors.len()-1];
    self.freelist.push(last);
    self.size -= 1;
    let mut temp = None;
    std::mem::swap(&mut temp, &mut self.nodes[last]);
    temp.unwrap().item
  }//delmax


}//RedBlackTree
