use scalar_types::{ding, on_off, print_array, print_difference, print_distance};

fn main() {
    // // unsigned int
    // let x: u32 = 23;
    // let y: u8 = 2;
    // // signed int
    // let z: i8 = 23;
    // let v: i64 = -234_000_000;
    //
    // // floats
    // let pi: f32 = 3.14159;
    // let tau: f64 = 6.28318530717;
    //
    // // suffix a literal with a type
    // let m  = 23u16;
    // let n = 2.53f32;
    // // we can use underscore to improve readability
    // let q = -121_i8;
    // let p = 123_u8;
    //
    // // bool
    // let isTrue: bool = true;
    // // casting a boolean
    // let isTrueInt = isTrue as u8;
    //
    // // char
    // let character: char = 'A';
    // let emoji_char: char = '❤';
    //
    // // tupple
    // let info: (u8, f64, i32) = (1, 3.3, 999); // tupple element should not be more than 12 else tupple will lose its purpose
    //
    // // first way of accessing elements in tupples
    // let jets = info.0;
    // let fuel = info.1;
    // let ammo = info.2;
    //
    // // second way to access members of a tupple
    // let(jets, fuel, ammo) = info;
    //
    // // Array
    // let buf = [1, 2, 3];
    // let buf = [0; 3]; // thus, [value, how many you want]
    // // type annotation for an array
    // let buf: [u8; 3] = [1, 2, 3];
    //
    // // index values in array
    // let i = buf[0];
    // // Array has a max size of 32, above which the loss their values

    /***
    Exercise
     */
    let coords: (f32, f32) = (6.3, 15.0);
    print_difference(coords);

    let coords_arr: [f32; 2] = [coords.0, coords.1];              // create an array literal out of parts of `coord` here
    print_array(coords_arr);        // and pass it in here (this line doesn't need to change)

    let series = [1, 1, 2, 3, 5, 8, 13];
    // 3. Make the `ding` function happy by passing it the value 13 out of the `series` array.
    // Use array indexing.  Done correctly, `cargo run` will produce the additional output
    // "Ding, you found 13!"
    //
    ding(series[6]);

    let mess = ([3, 2], 3.14, [(false, -3), (true, -100)], 5, "candy");
    // 4. Pass the `on_off` function the value `true` from the variable `mess`.  Done correctly,
    // `cargo run` will produce the additional output "Lights are on!" I'll get you started:
    //
    on_off(mess.2[1].0);

    // Challenge: Uncomment the line below, run the code, and examine the
    // output. Then go refactor the print_distance() function according to the
    // instructions in the comments inside that function.

    print_distance(coords);
}