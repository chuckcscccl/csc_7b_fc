//Circular Queue in C++

#include<iostream>
#include<vector>
#include<functional>
#include<concepts>     /* requires -std=c++20 */
#include<functional>
#include<optional>     /* requires -std=c++23 for monadic map/bind */
using namespace std;

#define None std::nullopt

template<typename T, int CAP = 64>
     requires std::copyable<T> && std::destructible<T> && (CAP > 2)
struct CircularQueue  {
private:
  vector<optional<T>> Q; 
  size_t front{0};            // size_t equiv to usize in rust
  size_t size{0};
  size_t index(size_t i) { //converts logical index to actual index
    return (front + i) % Q.size();
  }//index
public:
  CircularQueue() {
    //Q = vector<T>(CAP,None);   // doesn't compile but should
    Q.reserve(CAP);
    for(int i=0;i<CAP;i++) Q.push_back(None);
  }
  size_t len() { return size; }
  size_t current_capacity() { return Q.size(); }
  
  void resize() { // double capacity, allocate new vector.
    size_t newcap = Q.size() * 2;
    vector<optional<T>> Q2(newcap,None);
    for(int i=0;i<size;i++) std::swap<optional<T>>(Q[index(i)], Q2[i]);
    Q = move(Q2);
    front = 0;
  }
  
  void push_back(T x) {
    if (size>=Q.size()) resize();
    Q[index(size)] = move(x);   // don't need to place it in optional object
    size+=1;
  }//push_back

  void push_front(T x) {
    if (size>=Q.size()) resize();    
    front = (front + Q.size() - 1) % Q.size(); // wrap around to the left
    Q[front] = move(x);
    size += 1;
  }

  optional<T> pop_back() {
    if (size==0) return None; // std::None
    // optional<T> answer = move( Q[index(size-1)] );  // not possible in rust
    optional<T> answer = None;
    std::swap<optional<T>>(answer, Q[index(size-1)]);
    size--;
    return answer;
  }//pop_back

  optional<T> pop_front() {
    if (size==0) return None; // std::None
    optional<T> answer = None;
    std::swap<optional<T>>(answer, Q[index(0)]);
    front = (front + 1) % Q.size();  // wraps front to right
    size--;
    return answer;
  }//pop_back

  T& operator [] (size_t i) { // zero-overhead access
    return Q[index(i)].value();  // could crash
  } 
  
  optional<T> operator () (size_t i) { // checked access for dweebs
    if (i<size && Q[index(i)].has_value()) return Q[index(i)].value();
    else return None;  // implicitly copies.
  }

  void mapfun(function<void(T&)> f) {
    for(int i=0;i<size;i++) f(Q[index(i)].value());
  }

  // can't have optional<T&> in C++ - references not allowed for optional
};

int main() {
  CircularQueue<int> cq;
  for(int i=0;i<100;i++) {
    cq.push_back(i*2);
    cq.push_front(i*2 + 1);
  }
  for(int i=0;i<50;i++) {
    cq.pop_front();
    cq.pop_back();
  }
  cq.mapfun([](int& x){cout << x << " "; });
  cout << "\nsize: " << cq.len() << endl;
  cout << "capacity: " << cq.current_capacity() << endl;
  cout << cq(5).value() << endl;
  cq[5] = 99999;
  cout << cq(5).value() << endl;
  return 0;
}
