fn main() {
    // string slice (str)
    let hello_world = "Hello, Word!";

    // borrowed string slice (&str)
    let name: &str = "Matthew";

    // static string slice (will exist in the lifetime of the program
    let hello_world: &'static str = "Hello, World";

    // String (with capital S as String) can be modified unlike &str
    // we can create String from &str like:
    let msg = "ab🥰".to_string();
    // or
    let msg = String::from(hello_world);

}
