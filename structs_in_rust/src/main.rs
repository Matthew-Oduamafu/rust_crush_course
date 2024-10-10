fn main() {
    let fox = RedFox {
        enemy: true,
        life: 70,
    };

    let mut fox = RedFox::new();
    let life_left = fox.life;
    fox.enemy = false;
    // fox.some_method();

    let robot = Robot {};
    robot.run();
}

struct RedFox {
    enemy: bool,
    life: u8,
}

trait Noisy {
    fn get_noise(&self) -> &str;
}

// implementing a trait
impl Noisy for RedFox {
    fn get_noise(&self) -> &str {
        "Meow?"
    }
}

impl RedFox {
    fn new() -> Self {
        Self {
            enemy: true,
            life: 70,
        }
    }

    // associated functions
    fn function() {}

    // methods (care in Self)
    // fn move(self)...
    // fn borrow(&self)...
    // fn mut_borrow(&mut self)...
}

// traits help us write generic functions
fn print_noise<T: Noisy>(item: T) {
    println!("{}", item.get_noise())
}

/// traits can have default behavior too
trait Run {
    fn run(&self) {
        println!("I'm running!");
    }
}
struct Robot {}
impl Run for Robot {}