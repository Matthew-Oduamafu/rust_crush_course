fn main() {
    let quit_program = false;
    println!("{}", quit_program);
    let j = 'J';
    println!("{}", j);

    let (bunnies, carrots) = (8, 50);
    println!("{bunnies} can eat {carrots}");

    // the const
    const WARP_FACTOR: i32 = 89;
    println!("The const value {WARP_FACTOR}");
}
