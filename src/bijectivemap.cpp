#include<iostream>
#include<cstdio>         /* not everything old is bad */
#include<memory>         /* but new stuff is usually better */
#include<concepts>
#include<optional>       /* requires -std=c++23 for monadic combinators */
#include<functional>
#include<string>
#include<vector>
#include<unordered_map>
#include<tuple>
using namespace std;     /* compiled with g++-13 -std=c++23 on M2 Mac */

/*           Bijective Hashmap Without Cloning

Your assignment is to implement a bi-directional or two-way hashmap
These maps enforce a one-to-one relationship between keys and values.
There cannot be duplicate keys nor duplicate values.  The value can be
looked up with the key and the key with the value.  Both the value and
key can be hashed.

The usual way of implementing this data structure is to use two hashmaps
underneath, one in each direction: HashMap<KT,VT> and HashMap<VT,KT> where
KT, VT are the types of keys and values.  But this requires either cloning
the keys and values, or have multiple pointers to the same keys/values.

Most implementations of this data structure that you would find in
Rust would also assume that KT and VT implement the Clone trait, or
use Rc (reference counter) to have multiple pointers to the same
objects.  But cloning is clearly inefficient and Rc can potentially
compromise the safety of Rust.

My implementation below in C++ does not require cloning (but also
can't prevent it since C++ doesn't distinguish cloning from copying).
You are to write an equivalent version in Rust that does not require
Clone or Rc (or Arc).  

YOU MUST COMPLETE THE RUST VERSION OF THIS PROGRAM AS INSTRUCTED.

I've tried to use the latest features of C++ to make this program
resemble Rust - you can't compile this program without the newest C++
compiler (g++-13).  I've even aliased the typenames for vector and
unordered_map so they have rust names (Vec and HashMap).  Of course
the Rust program cannot look exactly like C++.  There are things you
can do in C++, such as indexing values with unsigned integers, that
you can't in Rust. When possible, take advantage of pattern matching
in Rust.  But the *algorithm* and structure of your program must be
based on this program.  I will be able to tell instantly if you got
some junk off the web or an AI.

The algorithm of this program essentially implements a hash table
partly from scratch.  I also have two hashmaps, but the keys of these
maps are integers (type size_t in modern C++ - equivalent to usize in
Rust).  One map (keys) stores keys and the *locations* of the
corresponding values in the other map.  Likewise, the other map (vals)
stores values and the locations of the corresponding keys in the keys
map.  Given a key and a value, I first get their hash values from the
built-in hashing functions of C++ (std::hash<>). Call these hk for
hash(key) and hv for hash(value).  These are then used as the KEYS for
the two hashmaps.  Because hash collisions can still happen, each
hashmap location contains a vector of tuples.  The keys map contains
vectors of tuples of the form (k,vr,vc). The key is k, vr is the *hash
key* of the corresponding value in the other map (vals), and vc is the
vector index of the corresponding entry in the other map.  Likewise,
the vals map contains vectors of tuples of the form (v,kr,kc) where v
is a value, kr is a hash key of the keys map, and kc is the vector
index of the tuple that holds the corresponding key.  When looking up
a value by key, we first locate the entry for the key in the keys map,
(k,vr,vc).  Then we use this information to lookup the value, which is
at vals[vr][vc] (think of the hashmap of vectors as a 2D array).  The
algorithm to lookup the key with the value is symmetrical.  The
program looks long because there are twin versions of many procedures.
If you figure out how to write one you'll definitely be able to write
the other.

The rust program will also not be perfectly equivalent because of 
certain limitations of C++. For example, the C++ `optional` type
cannot contain a reference, and I'm forced to return a *copy* of the
value in an optional when looking up a value or key.  The equivalent
of these methods (get_by_key, get_by_val) should return Option<&VT>
and Option<&KT> in Rust.  

Other differences between C++ and Rust that pertains to this program:

  - there's no distinction between l-value and r-value references in rust
    but there certainly is between immutable and mutable references.
    In C++, although you can have a const T&, there's not as much
    significance to it.

  - Although the Index trait is implemented for Rust's HashMap, and
    you can say mymap[&key] to access the value associated with the
    key (if you're sure it exists), you cannot assign to it like in
    C++ (it's immutable).  To change something at that entry you can
    say mymap.get_mut(&key).map(|x|{x+=1;}); assumming the entry is an int.
    Alternatively, if your key implements Copy, you can say

        mymap.entry(key).and_modify(|x|{x+=1;});

    Yes, entry is another MONAD MONSTER! (and it's not even Holloween).

I've also written out a skeleton for you in Rust and the signatures of
the functions you must implement.  Your program must continue from 
this skeleton.
*/


#define None std::nullopt;

template<typename T>
using Vec = std::vector<T>;  // type alias Vec<T>

template<typename K, typename V>
using HashMap = std::unordered_map<K,V>;  // type alias HashMap<K,V>

template<
  typename KT,         /* type of keys */
  typename VT,         /* type of values*/
  typename HK = std::hash<KT>,  /* hasher for keys */
  typename HV = std::hash<VT>   /* hasher for values */
  >
  requires std::equality_comparable<KT>
        && std::equality_comparable<VT>
        && std::movable<KT>
        && std::movable<VT>
        && std::destructible<KT>
        && std::destructible<VT>

struct bijective_map {
private:
  HK keyhash{};  // {} applies default constructor
  HV valhash{};
  size_t size{0};
  HashMap<size_t,Vec<tuple<KT,size_t,size_t>>> keys{};
  HashMap<size_t,Vec<tuple<VT,size_t,size_t>>> vals{};  
  //Vec<tuple<KT,size_t,size_t>> keys[ROWS];
  //Vec<tuple<VT,size_t,size_t>> vals[ROWS];
public:
  // default constructor suffice  -  write a fn new() -> Self in Rust

  size_t len() { return size; }

  // define get methods first because they're the easiest (non-mutating)
  optional<const VT> get_by_key(const KT& key) {
    try {
      auto hk = keyhash(key);
      auto row = keys[hk];  // throws std::out_of_range if key not found
      for(auto& [k,vr,vc] : row) {  // for (k,vr,vc) in rows.iter() (rust)
        if (k==key) {
          VT v; size_t kr, kc;
          tie (v,kr,kc) = vals[vr][vc]; // let (v,_,_) = ..
          return v;   // Some(v)
        }
      }//for
    }
    catch(out_of_range e) { }
    return None;   // None is alias for std::nullopt
  }//get_by_key for l-value references to keys
  
  optional<const VT> get_val(KT&& key) { // r-value ref version of get_by_key
    return get_by_key(key);
  } // can't call get_by_key("larz") because "larz" is not an l-value

  // the following is entirely symmetric to get_by_key
  optional<const KT> get_by_val(const VT& val) {
    try {
      auto hv = valhash(val);
      auto row = vals[hv];  
      for(auto& [v,kr,kc] : row) {
        if (v==val) {
          KT k; size_t vr, vc;
          tie (k,vr,vc) = keys[kr][kc];
          return k;
        }
      }//for
    }
    catch(out_of_range e) { }
    return None;  
  }//get_by_val

  optional<const KT> get_key(VT&& val) {  // r-value ref version of get_by_key
    return get_by_val(val);
  }

  // The following are defined first because they're required by method set
  optional<tuple<KT,VT>> take_by_key(const KT& key) {
    KT k; VT v; size_t kr, kc, vr, vc;
    auto hk = keyhash(key);
    auto row = keys[hk];  // vector of keys (hash collisions)
    auto flen = row.size();
    int i = -1;  // don't try this trick in rust - you'll get crushed
    while (++i < flen) {
      if (get<0>(row[i]) == key) {break;} // found key!
    }//while  (c++ has a weird way of getting the elements of a tuple)
    if (i==flen) { return None; }
    if (i+1<flen) { // delete from vector by swapping with the last element
      std::swap(row[i],row[flen-1]);  // in rust, just vector.swap(i,flen-1)
      auto ir = get<1>(row[i]);  // must also adjust location of swapped value
      auto ic = get<2>(row[i]);
      get<2>(vals[ir][ic]) = i; // get returns l-value reference, in rust:
    }                           // keys.entry(ir).and_modify(|v|v[ic]=i);
    tie (k,vr,vc) = row.back();   // now pop the value : O(1) delete
    row.pop_back();  // pop_back returns void (rust pop returns Option)
    // got key and location of value, now find value...
    auto blen = vals[vr].size();
    if (vc+1<blen) {
      std::swap(vals[vr][vc], vals[vr][blen-1]);
      auto ir = get<1>(vals[vr][vc]);
      auto ic = get<2>(vals[vr][vc]);
      get<2>(keys[ir][ic]) = i;  
    }
    tie (v,kr,kc) = vals[vr].back();
    vals[vr].pop_back();
    size--;
    return make_tuple(k,v);  // Some((k,v))
  }//take_by_key
  optional<tuple<KT,VT>> take_val(KT&& key) { return take_by_key(key); }

  optional<tuple<KT,VT>> take_by_val(const VT& val) {
    KT k; VT v; size_t kr, kc, vr, vc;
    auto hv = valhash(val);
    auto row = vals[hv];  
    auto flen = row.size();
    int i = -1;
    while (++i < flen) {
      if (get<0>(row[i]) == val) {break;}
    }//while
    if (i==flen) { return None; }
    if (i+1<flen) {
      std::swap(row[i],row[flen-1]); 
      auto ir = get<1>(row[i]);  
      auto ic = get<2>(row[i]);
      get<2>(keys[ir][ic]) = i;  
    }
    tie (v,kr,kc) = row.back();  
    row.pop_back(); 
    auto blen = keys[kr].size();
    if (kc+1<blen) {
      std::swap(keys[kr][kc], keys[kr][blen-1]);
      auto ir = get<1>(keys[kr][kc]);
      auto ic = get<2>(keys[kr][kc]);
      get<2>(vals[ir][ic]) = i;
    }
    tie (k,vr,vc) = keys[kr].back();
    keys[kr].pop_back();
    size--;
    return make_tuple(k,v);
  }//take_by_key
  optional<tuple<KT,VT>> take_key(VT&& val) { return take_by_val(val); }

  // finally, we can write the set method, which either inserts or
  // changes a key-value pair, making sure that the map stays bijective.
  // the function returns the previous key-value pair, if there was one
  optional<tuple<KT,VT>> set(KT key, VT val) { // note these aren't refs
    // make sure vectors exist at locations:
    auto hk = keyhash(key);
    auto hv = valhash(val);
    if (!keys.contains(hk)) {
      vector<tuple<KT,size_t,size_t>> v{};
      keys[hk] = move(v);
    } // in rust, just do keys.entry(hk).or_default();
    if (!vals.contains(hv)) {
      vector<tuple<VT,size_t,size_t>> v{};
      vals[hv] = move(v);      
    }
    optional<tuple<KT,VT>> answer = None;  // to be returned, default none
    KT k1, k2;  VT v1, v2;
    auto tkey = take_by_key(key); // delete previous associations
    auto tval = take_by_val(val);
    // following code should be simpler in Rust because of pattern matching:
    if (tkey.has_value() && tval.has_value()) {
      tie (k1,v1) = tkey.value();
      tie (k2,v2) = tval.value();
      answer = {k2,v1};
    }
    else if (tkey.has_value()) {
      answer = tkey;
    }
    else if (tval.has_value()) {
      answer = tval;
    }
    // insert new association:
    auto kc = keys[hk].size();
    auto vc = vals[hv].size();
    keys[hk].push_back({key,hv,vc});
    vals[hv].push_back({val,hk,kc});
    size++;
    return answer;
  }//set
  
}; // bijective map


int main() {
  bijective_map<string,int> daynum;
  string days[] = {"Monday","Tuesday","Wednesday","Thursday","Friday","Saturday","Sunday"};
  for(int i=0;i<7;i++) {daynum.set(days[i],i+1);}

  // a Curried lambda function:
  auto println = [](string&& p){
    return [=](auto& n) { cout << p << " " << n << endl; return 0; };
  };  // in rust: let cout = |p|move |n|println!("{} {}",&p,&n);

  auto option = daynum.get_val("Wednesday");
  cout << option.value() << endl; // .value() = .unwrap() 

  // But what if you misspelled Wednesday? Need if (option).. but better
  // to do real monadic error handling with .map (.transform in C++23):
  
  daynum.get_val("Thursday").transform(println("Thursday is day"));

  daynum.get_key(7).transform(println("day 7 is"));
  
  auto opt = daynum.set("Sunday",1); // must also erase association for Monday
  opt.transform([](auto& v) {
    cout << "set Sunday returned " <<get<0>(v) << "," << get<1>(v) << endl;
    return 0;  // C++ doesn't have the unit type so need to return something
  });
  printf("size : %d\n", daynum.len()); // now 6 because no more Monday

  for(int i=0;i<6;i++) {
     opt = daynum.set(days[i],i+2);
     opt.transform([](auto& rv) {   
       cout << "set returned " <<get<0>(rv) << "," << get<1>(rv) << endl;
       return 0;
     });
  }//for

  printf("size : %d\n", daynum.len()); // better be 7

  return 0;
}//main
// Program by Chuck Liang, on github under MIT license.
