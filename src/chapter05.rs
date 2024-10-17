#[test]
fn test511() {
    fn main() {
        // Use as many approaches as you can to make it work
        let x = String::from("Hello world");
        let y = x.clone();
        println!("{}, {}",x, y);
    }
}

#[test]
fn test512(){
    // Don't modify code in main!
    fn main() {
        let s1 = String::from("Hello world");
        let s2 = take_ownership(s1);

        println!("{}", s2);
    }

    // Only modify the code below!
    fn take_ownership(s: String) -> String {
        println!("{}", s);
        s
    }
}

#[test]
fn test513(){
    fn main() {
        let s = give_ownership();
        println!("{}", s);
    }

    // Only modify the code below!
    fn give_ownership() -> String {
        let s = String::from("Hello world");
        let _s = s.clone().into_bytes();
        s
    }
}

#[test]
fn test514(){
    // Fix the error without removing any code
    fn main() {
        let s = String::from("Hello World");
        print_str(&s);
        println!("{}", s);
    }

    fn print_str(s: &String) {
        println!("{}", s);
    }
}

#[test]
fn test515(){
    fn main() {
        let x = (1, 2, (), "hello");
        let y = x;
        println!("{:?}, {:?}", x, y);
    }

}

#[test]
fn test516(){
    fn main() {
        let mut x = Box::new(5);
        let y = &mut *x;
        *y = 4;
        assert_eq!(*x, 5);

        println!("Success!");
    }

}

#[test]
fn test521(){
    fn main() {
        let x = 5;
        // Fill the blank
        let p = &x;

        println!("the memory address of x is {:p}", p);
    }
}

#[test]
fn test522(){
    fn main() {
        let x = 5;
        let y = &x;

        // Modify this line only
        assert_eq!(5, *y);
        println!("Success!");
    }
}

#[test]
fn test523(){
    fn main() {
        let mut s = String::from("hello, ");

        borrow_object(&s);

        println!("Success!");
    }
    fn borrow_object(s: &String) {}

}

#[test]
fn test524(){
    fn main() {
        let mut s = String::from("hello, ");

        push_str(&mut s);

        println!("Success!");
    }

    fn push_str(s: &mut String) {
        s.push_str("world");
    }
}

#[test]
fn test525(){
    fn main() {
        let mut s = String::from("hello, ");

        // Fill the blank to make it work
        let p = &mut s;
        p.push_str("world");

        println!("Success!");
    }
}

#[test]
fn test526(){
    fn main() {
        let c = '中';

        let r1 = &c;
        // Fill the blank，dont change other code
        let r2 = &c;

        assert_eq!(*r1, *r2);

        assert_eq!(get_addr(r1), get_addr(r2));

        println!("Success!");
    }

    fn get_addr(r: &char) -> String {
        format!("{:p}", r)
    }
}

#[test]
fn test527(){
    fn main() {
        // Fix error by modifying this line
        let mut s = String::from("hello, ");

        borrow_object(&mut s);

        println!("Success!");
    }
    fn borrow_object(s: &mut String) {}

}