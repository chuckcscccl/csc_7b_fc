// circular queue

pub struct CQ<T> {
  q : Vec<Option<T>>,
  front : usize,
  size : usize,
}

impl<T> CQ<T> {
  pub fn new(n:usize) -> Self {
    let mut q = Vec::with_capacity(n);
    //q = vec![None;n];  // won't compile because T can't be cloned
    q.resize_with(n,||None);
    CQ { q: q, front: 0, size: 0,}
  }

  // converts "virtual index" i to "actual index" 
  fn index(&self, i:usize) -> usize {
    (self.front+i) % self.q.len()
  }

  // double capacity
  fn resize(&mut self) {
    let newcap = self.q.len() * 2;
    let mut q2 = Vec::with_capacity(newcap);
    q2.resize_with(newcap,||None);
    for i in 0..self.size {
      let k = self.index(i);
      std::mem::swap(&mut q2[i], &mut self.q[k]);
      //q2[i] = q[index(i)]
    }
    self.q = q2;
    self.front = 0;
  }//resize

  pub fn push(&mut self, x:T) {
    // move front one space to the left
    if self.size>=self.q.len() { self.resize(); }
    self.front = (self.front + self.q.len() -1) % self.q.len();
    self.q[self.front] = Some(x);
    self.size+=1;
  } //push

  pub fn pop(&mut self) -> Option<T> {
    if self.size==0 { return None; }
    let mut temp = None;
    std::mem::swap(&mut temp, &mut self.q[self.front]);
    self.front = (self.front+1)%self.q.len();
    self.size -= 1;
    temp
  }// pop

  pub fn peek(&self) -> Option<&T> {
    if self.size==0 { None }
    else { self.q[self.front].as_ref() }
  }//peek

  pub fn enqueue(&mut self, x:T) {
    if self.size>=self.q.len() { self.resize(); }
    self.q[self.size] = Some(x);
    self.size += 1;
  }//enqueue

  pub fn dequeue(&mut self) -> Option<T> {
    if self.size==0 { return None; }
    let mut temp = None;
    let k = self.index(self.size-1);
    std::mem::swap(&mut temp, &mut self.q[k]);
    self.size -= 1;
    temp
  }

  pub fn len(&self) -> usize { self.size }

  pub fn get(&self, i:usize) -> Option<&T> {
    if i>=self.size { None }
    else { self.q[self.index(i)].as_ref() }
  }//get

  pub fn get_mut(&mut self, i:usize) -> Option<&mut T> {
    if i>=self.size { None }
    else {
      let k = self.index(i);
      self.q[k].as_mut()
    }
  }//get_mut

  pub fn set(&mut self, i:usize, x:T) -> Option<T> {
    if i>=self.size { None }
    else {
      let mut temp = Some(x);
      let k = self.index(i);
      std::mem::swap(&mut temp, &mut self.q[k]);
      temp
    }
  }//set

  pub fn swap(&mut self, i:usize, k:usize) -> bool {
    if i>=self.size || k>=self.size { false }
    else {
       let (ri,rk) = (self.index(i), self.index(k));
       self.q.swap(ri,rk);
       true
    }
  }//swap

  pub fn map<F>(&self, mapfun:&mut F) where F:FnMut(&T) {
    for i in 0..self.size {
      self.q[self.index(i)].as_ref().map(|x|mapfun(x));
    }
  }//map

 pub fn insert(&mut self, i:usize, x:T) -> bool {
   if i>=self.size { return false; }
   if self.size >= self.q.len() { self.resize(); }
   if i < self.size/2 {  //shuffle left
     self.front = (self.front + self.q.len() - 1) % self.q.len();
     for i in 1..i {
       let k = self.index(i-1);
       let k2 = self.index(i);
       self.q.swap(k2,k);
     }
   }
   else {  // shuffle right
     let mut k = self.size;
     while k > i {
       let k1 = self.index(k);
       let k2 = self.index(k-1);
       self.q.swap(k1,k2);
       k -= 1;
     }//while
   }
   let k = self.index(i);
   self.q[k] = Some(x);
   self.size += 1;
   true
 }//insert

 pub fn remove(&mut self, i:usize) -> Option<T> {
   if i>=self.size { return None; }
   let mut answer = None;
   let mut k = self.index(i);
   if i<self.size/2 { // shuff left
     std::mem::swap(&mut answer, &mut self.q[k]);
     k = i;
     while k>0 {
       let k1 = self.index(k);
       let k2 = self.index(k-1);
       self.q.swap(k1,k2);
       k -= 1;
     }//while
     self.front = (self.front+1) % self.q.len();
   }
   else { //shuffle right
     std::mem::swap(&mut answer, &mut self.q[k]);
     k = i;
     while k+1 < self.size {
       let k1 = self.index(k);
       let k2 = self.index(k+1);
       self.q.swap(k1,k2);
       k += 1;
     }
   }
   self.size -= 1;
   answer
 }//remove

}// main impl CQ


use std::ops::{Index, IndexMut};
// q: CQ  q[i]  q[i] = 4;

impl<T> Index<usize> for CQ<T> {
  type Output = T;
  fn index(&self,i:usize) -> &Self::Output {
     return self.q[self.index(i)].as_ref().unwrap();
  }//index
}

impl<T> IndexMut<usize> for CQ<T> {
  fn index_mut(&mut self, i:usize) -> &mut Self::Output {
     let k = self.index(i);
     self.q[k].as_mut().unwrap()
  }//index_mut
}

///// immutable iterator implementation
pub struct CQIter<'lt,T> {
  q : &'lt CQ<T>,
  index: usize,
}

impl<'lt,T> Iterator for CQIter<'lt,T> {
  type Item = &'lt T;
  fn next(&mut self) -> Option<Self::Item> {
    if self.index >= self.q.size { None }
    else {
      self.index += 1;
      self.q.get(self.index-1)
    }
  }//next
}

impl<T> CQ<T> {
  pub fn iter<'lt>(&'lt self) -> CQIter<'lt,T> {
    CQIter { q : self, index:0 }
  }
}//

impl<'lt,T> IntoIterator for &'lt CQ<T> {
  type Item = &'lt T;
  type IntoIter = CQIter<'lt,T>;
  fn into_iter(self) -> Self::IntoIter {
    self.iter()
  }
}

///// mutable iterator implementation
pub struct CQMutIter<'lt,T> {
  q : &'lt mut CQ<T>,
  index: usize,
}

impl<'lt,T> Iterator for CQMutIter<'lt,T> {
  type Item = &'lt mut T;
  fn next(&mut self) -> Option<Self::Item> {
    if self.index >= self.q.size { None }
    else {
      self.index += 1;
      let k = self.q.index(self.index-1);
      let ptr = self.q.q.as_mut_ptr();
      unsafe { (*ptr.add(k)).as_mut() }
      //self.q.get(self.index-1)
    }
  }//next
}

impl<T> CQ<T> {
  pub fn iter_mut<'lt>(&'lt mut self) -> CQMutIter<'lt,T> {
    CQMutIter { q : self, index:0 }
  }
}//

impl<'lt,T> IntoIterator for &'lt mut CQ<T> {
  type Item = &'lt mut T;
  type IntoIter = CQMutIter<'lt,T>;
  fn into_iter(self) -> Self::IntoIter {
    self.iter_mut()
  }
}

//////////// ordered interpretations...
impl<T:Ord> CQ<T> {
  pub fn linear_search(&self, x:&T) -> Option<usize> {
    for i in 0..self.size {
      match &self.q[self.index(i)] {
        Some(y) if y==x => { return Some(i); },
	_ => {},
      }//match
    }
    None
  } // linear_search

  pub fn is_sorted(&self) -> bool {
     for i in 1 .. self.size {
       match (&self.q[self.index(i)], &self.q[self.index(i-1)]) {
         (Some(x), Some(y)) if x<y => { return false; },
	 _ => {},
       }//match
     }
     true
  }// is_sorted

  pub fn insert_sorted(&mut self, x:T) {
    self.push(x);
    let mut i = 0;
    while i+1 < self.size {
      let k1 = self.index(i);
      let k2 = self.index(i+1);
      if &self.q[k1] > &self.q[k2] { self.q.swap(k1,k2); }
      else { break; }
      i += 1;
    }//while
  }//insert_sorted

  pub fn binary_search(&self, x:&T) -> Option<usize> {
    let (mut min, mut max) = (0, self.size);
    while min < max {
      let mid = min + (max-min)/2;
      match &self.q[mid] {
        Some(y) if y==x => { return Some(mid); },
	Some(y) if x<y => { max = mid; },
	Some(_) => { min = mid+1; },
	_ => { return None; }  // should never get here
      }//match
    }//while
    None
  }//binary_search
}


fn main() {
  let mut Q:CQ<i32> = CQ::new(512);
  for x in [2,4,6,8,10,12,14,16] {Q.push(x);}
  for x in [1,3,5,79,11,13,15,17] {Q.enqueue(x);}
  Q.pop();
  Q.dequeue();
  Q[2] = Q[2] * Q[2];
  //Q.map(&mut |x|{print!("{} ",x);});  println!();
  for x in Q.iter() { print!("{} ",x); }  println!();

  Q.insert(3,99);
  Q.insert(8,199);
  Q.remove(2);

  for x in &mut Q { *x *= 10; }

  for x in /*Q.iter()*/ &Q { print!("{} ",x); }  println!();
} //main
