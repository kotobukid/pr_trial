fn say_goodbye() -> () {
    println!("goodbye")
}

fn say_hello(name: &str) -> () {
    println!("Hello, {name}!")
}

pub fn add_u8(a: u8, b: u8) -> u8 {
    a + b
}

fn main() {
    println!("Hello, world!");
    println!("Hello, world!(2)");
    say_hello("Taro");

    say_goodbye()
}

#[cfg(test)]
mod sample_test {
    use crate::*;

    #[test]
    fn test1() {
        assert_eq!(8, add_u8(4, 4))
    }
    #[test]
    fn test_to_fail() {
        assert_eq!(7, add_u8(3, 4))
    }
}