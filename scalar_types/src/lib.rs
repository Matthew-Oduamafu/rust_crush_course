pub fn print_difference(params: (f32, f32)) {
    println!("Difference between {} and {} is {}", params.0, params.1, (params.0 - params.1).abs());
}

pub fn print_array(a: [f32; 2]) {
    println!("The coordinates are ({}, {})", a[0], a[1]);
}

pub fn ding(x: i32) {
    if x == 13 {
        println!("Ding, you found 13!");
    }
}

pub fn on_off(val: bool) {
    if val {
        println!("Lights are on!");
    }
}

pub fn print_distance(z: (f32, f32)) {
    let (x, y) = z;
    println!(
        "Distance to the origin is {}",
        (x.powf(2.0) + y.powf(2.0)).sqrt());
}