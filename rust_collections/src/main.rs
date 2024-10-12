use std::collections::{HashMap, HashSet};

fn main() {
    // declaring a vector
    let mut v: Vec<i32> = Vec::new();
    v.push(2);
    v.push(4);
    v.push(6);

    let x = v.pop();  // x is 6 [pop() removes the item at the end and returns it]
    println!("{}", v[1]);
    match x {
        Some(value) => println!("{:?}", value),
        None => println!("No value found"),
    }

    // ergonomic way of creating vectors
    let mut v = vec![2, 4, 6];

    // hashMap
    let map: HashMap<&str, i32> = HashMap::new();
    let mut h: HashMap<u8, bool> = HashMap::new();
    h.insert(5, true);
    h.insert(6, false);

    // we use the remove mthd to get a value
    let have_five = h.remove(&5).unwrap();

    // HashSet
    let mut s: HashSet<u8> = HashSet::new();
    s.insert(4);
    s.insert(3);
    s.insert(30);
    s.insert(30);
    let last_member = s.remove(&2);

    let mut set: HashSet<i32> = HashSet::new();
    set.insert(1);
    set.insert(2);
    set.insert(3);
    set.insert(1); // Duplicate value, will not be added

    println!("{:?}", set); // Output: {1, 2, 3}

    let mut set: HashSet<i32> = HashSet::new();
    set.insert(1);
    set.insert(2);
    set.insert(3);

    if set.contains(&2) {
        println!("Set contains 2");
    } else {
        println!("Set does not contain 2");
    }


    let mut set: HashSet<i32> = HashSet::new();
    set.insert(1);
    set.insert(2);
    set.insert(3);

    set.remove(&2);

    println!("{:?}", set); // Output: {1, 3}


    let mut set: HashSet<i32> = HashSet::new();
    set.insert(1);
    set.insert(2);
    set.insert(3);

    for value in &set {
        println!("{}", value);
    }
}
