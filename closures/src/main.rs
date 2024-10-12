fn main() {
    let add = |x, y| { x + y };
    let sum = add(1, 2);
    println!("{}", sum);

    // parameters are not compulsory for closures
    let x = 5;
    let y = 3;
    let sub = || { x - y };
    println!("{}", sub());

    let s = "🍓".to_string();
    let f = || {
        println!("{}", s)
    };
    f();
    // for the closure above, it will only run if s is always within the same scope as the closure,
    // thus, the closure should never out live the variable it is referencing,
    // but the compiler won't let send this over to another thread

    // but luckily closure supports change of ownership
    // we can use the `move` keyword to move all variables that the closure depends on,
    // but doing this we move a closure over to another thread
    let f = move || {
        println!("{}", s)
    };
    f();

    let mut v = vec![2, 4, 6];
    v.iter()
        .map(|x| x * 3)
        .filter(|x| *x > 10)
        .fold(0, |acc, x| { acc + x });
}
