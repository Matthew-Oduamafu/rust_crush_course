fn main() {
    let mut num = 5;
    let msg: String;

    if num == 5 {
        msg = "five".parse().unwrap();
    } else if num == 4 {
        msg = "four".parse().unwrap();
    } else {
        msg = "unknown".parse().unwrap();
    }

    // if is not a statement, it's an expression, meaning the above can be written as below
    msg = if num == 5 {
        "five".parse().unwrap()
    } else {
        "other".parse().unwrap()
    };

    // loops

    loop {
        if num == 1 {
            break;
        }
        num = num - 1;
    }
    // breaking nested loop
    'bob: loop {
        loop {
            loop {
                break 'bob;
            }
        }
    }

    num = 10;
    // using continue
    loop {
        if num == 3 {
            continue;
        } else if num == 0 {
            break
        }
        num = num - 1;
    }

    num = 10;

    'bob2: loop {
        loop {
            if num == 3 {
                continue 'bob2;
            } else if num == 0 {
                break;
            }
        }
    }

    while num > 0 {
        num -= 1;
    }

    for i in 0..100 {
        println!("{}", i);
    }
    for num in [7, 8, 9].iter() {
        println!("{}", num)
    }

    let array = [(1, 2), (3, 4)];
    for (x, y) in array.iter() {
        println!("{}, {}", x, y)
    }
}
