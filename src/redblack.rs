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
  pub fn new_leaf(i:T, r:bool) -> Self {
     Node { item:i, left:NIL, right:NIL, red:r, }
  }
  pub fn is_red(&self) -> bool { self.red }
  pub fn set_red(&mut self, red:bool) {
    self.red = red;
  }
}

// convenient operations on Option
fn is_black<T>(nopt:&Option<Node<T>>) -> bool {
  nopt.as_ref().map(|n|!n.is_red()).unwrap_or(true)
}
fn set_black<T>(nopt: &mut Option<Node<T>>, black:bool) {
  nopt.as_mut().map(|n|n.set_red(!black));
}
fn get_left<T>(nopt:&Option<Node<T>>) -> usize {
  nopt.as_ref().map(|n|n.left).unwrap_or(NIL)
}
fn set_left<T>(nopt: &mut Option<Node<T>>, n:usize) {
  nopt.as_mut().map(|nd|nd.left = n);
}
fn get_right<T>(nopt:&Option<Node<T>>) -> usize {
  nopt.as_ref().map(|n|n.right).unwrap_or(NIL)
}
fn set_right<T>(nopt: &mut Option<Node<T>>, n:usize) {
  nopt.as_mut().map(|nd|nd.right = n);
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
    let newnode = Node::new_leaf(x,true);
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
	self.nodes[index].as_mut().map(|n|n.set_red(false));
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
      let mut deleted = self.delmax(current);
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
    let original_current = current;
    current = get_left(&self.nodes[current]);
    while current!=NIL {
      ancestors.push(current);
      self.nodes[current].as_ref().map(|n|current = n.right);
    }
    let last = ancestors.pop().unwrap();
    let last_left = get_left(&self.nodes[last]);
    if let Some(grandparent) = ancestors.pop() {
       set_right(&mut self.nodes[grandparent], last_left);
    }
    else { // immediate left node is deleted
       set_left(&mut self.nodes[original_current], last_left);
    }
    self.freelist.push(last);
    self.size -= 1;
    let mut temp = None;
    std::mem::swap(&mut temp, &mut self.nodes[last]);
    temp.unwrap().item
  }//delmax

  ///// Rotations
  fn LL(&mut self, index:usize) { // "right rotation"
    let left = get_left(&self.nodes[index]);
    let right = get_right(&self.nodes[index]);
    let ll = get_left(&self.nodes[left]);
    let lr = get_right(&self.nodes[left]);    
    self.nodes.swap(left,index);
    set_left(&mut self.nodes[index], ll);
    set_left(&mut self.nodes[left], lr);
    set_right(&mut self.nodes[left], right);
    set_right(&mut self.nodes[index], left);
  }

  fn RR(&mut self, index:usize) { // "left rotation"
    let right = get_right(&self.nodes[index]);
    let left = get_left(&self.nodes[index]);
    let rl = get_left(&self.nodes[right]);
    let rr = get_right(&self.nodes[right]);
    self.nodes.swap(index,right);

    set_black(&mut self.nodes[index],false);
    set_black(&mut self.nodes[right],true);
    //set_black(&mut self.nodes[rr],false);   // ???

    set_right(&mut self.nodes[index],rr);
    set_right(&mut self.nodes[right],rl);
    set_left(&mut self.nodes[right],left);
    set_left(&mut self.nodes[index],right);
  }

}//RedBlackTree

/*
      x          y
     / \        / \
    y   R      LL  x 
   / \            / \
  LL LR          LR  R

      y          x
     / \        / \
    x  RR      L   y 
   / \            / \
  L  RL          RL RR
*/
