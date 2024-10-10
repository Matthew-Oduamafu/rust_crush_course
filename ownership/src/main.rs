fn main() {
    let s1 = String::from("abc");
    let s2 = s1;
    println!("{}", s1); // Error because the value at s1 was moved to s2, so s1 appears as uninitialized
    println!("{}", s2);

    // to make a copy instead of changing the owner, we do this
    let s1 = String::from("abc");
    let s2 = s1.clone();
    println!("s1: {}, s2: {}", s1, s2);

    // so when we create a function and pass in a variable, it means we change ownership
    // let s1 = String::from("ABC");
    // do_stuff(s1);
    // s1 is now uninitialized and we can't use it
    // that's why we use borrowing and referencing

    let s1 = String::from("Abc");
    do_stuff(&s1);
    // references default to immutable
    // but if we make a mutable reference to a mutable value, then we can use the reference to change the value as well
    let mut s1 = String::from("abc");
    do_work(&mut s1);
    println!("{}", s1)
}

fn do_stuff(s: &String) {
    println!("{}", s);
}
fn do_work(s: &mut String) {
    s.insert_str(0, "Hi, "); // dereference is done automatically
    // to re-assign we need to deference manually
    *s = String::from("replacement");
}