use std::fs::File;

fn main() {
    println!("Hello, world!");

    let color = Color::Red;
    let item = DispenserItem::Empty;
    let item = DispenserItem::Ammo(69);
    let item = DispenserItem::Things("hat".to_string(), 7);
    let item = DispenserItem::Place { x: 24, y: 48 };

    // we use if let to match one variant
    let mut v: Vec<i32> = Vec::new();
    v.push(2);

    let my_variable = v.pop();
    if let Some(x) = my_variable {
        println!("value is {}", x)
    }

    // if we have to match all cases
    match my_variable {
        None => {
            println!("No value");
        }
        Some(x) => {
            println!("value is {}", x)
        }
    }
    // match expects you to take care of all possible cases
    // for default branch we use underscore (_)
    match my_variable {
        _ => {
            println!("default branch here")
        }
    }

    // branch arms can return
    // either all branch arms return nothing, or all branch arms return something of the same type
    let x = match my_variable {
        None => 42,
        Some(x) => x.pow(3),
    };

    let mut x: Option<i32> = None;
    x = Some(5);
    let is_some = x.is_some();
    let is_none = x.is_none();

    for i in x {
        println!("{}", i)
    }

    // Result enum
    let res = File::open("foo");
    // let f = res.expect("error message");
    // if res.is_ok() {
    //     let f = res.unwrap();
    // }
    match res {
        Ok(f) => {
            println!("{:?}", f);
        }
        Err(e) => {
            println!("{}", e)
        }
    }
}

// simple enum
enum Color {
    Red,
    Green,
    Blue,
}

// complex enum
enum DispenserItem {
    Empty,
    Ammo(u8),
    Things(String, i32),
    Place { x: i32, y: i32 },
}

// we can implement functions for an enum
impl DispenserItem {
    fn display(&self) {
        println!("Something from an enum")
    }
}