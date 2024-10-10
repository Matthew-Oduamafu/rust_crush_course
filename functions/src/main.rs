fn main() {
    let width = 4;
    let height = 7;
    let depth = 10;
    let area = area_of(width, depth);
    let volume = volume_of(width, depth, height);
    println!("Area of rectangle with length {depth} and width {width} is {area} sq units");
    println!("With height of {height}, volume is {volume} cubed units");
}

fn area_of(x: i32, y: i32) -> i32 {
    return x * y;
}

fn volume_of(x: i32, y: i32, h: i32) -> i32 {
    area_of(x, y) * h
}