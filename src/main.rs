fn say_goodbye() -> () {
    println!("goodbye")
}

fn say_hello(name: &str) -> () {
    println!("Hello, {name}!")
}

fn main() {
    println!("Hello, world!");
    println!("Hello, world!(2)");

    say_goodbye()
}
