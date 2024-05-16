fn say_goodbye() -> () {
    println!("goodbye")
}

fn say_hello(name: &str) -> () {
    println!("Hello, {name}!")
}

fn main() {
    println!("Hello, world!");

    say_goodbye()
}
