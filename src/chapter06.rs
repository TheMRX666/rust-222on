#[test]
fn test611() {
    // Fix error without adding new line
    fn main() {
        let s: &str = "hello, world";

        println!("Success!");
    }
}

#[test]
fn test612() {
    fn main() {
        let s: Box<str> = "hello, world".into();
        greetings(s)
    }

    fn greetings(s: Box<str>) {
        println!("{}", s);
    }
}

#[test]
fn test613() {
    // Fill the blank
    fn main() {
        let mut s = String::new();
        s.push_str("hello, world");
        s.push('!');

        assert_eq!(s, "hello, world!");

        println!("Success!");
    }
}

