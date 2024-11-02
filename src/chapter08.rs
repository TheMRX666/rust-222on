#[test]
fn test811() {
    enum Direction {
        East,
        West,
        North,
        South,
    }
    fn main(){
        let dire = Direction::South;
        match dire {
            Direction::East => println!("East"),
            Direction::South | Direction::North => { // Matching South or North here
                println!("South or North");
            },
            _ => println!("West"), // Handle other cases (like West)
        };
    }
}

#[test]
fn test812(){
    fn main(){
        let boolean = true;

        // Fill the blank with a match expression:
        let binary = match boolean {
            true => 1,
            false => 0,
        };

        assert_eq!(binary, 1);
        println!("Success!");
    }
}

#[test]
fn test813(){
    enum Message {
        Quit,
        Move { x: i32, y: i32 },
        Write(String),
        ChangeColor(i32, i32, i32),
    }

    fn show_message(msg: Message) {
        match msg {
            Message::Move { x: a, y: b } => { // match Message::Move
                assert_eq!(a, 1);
                assert_eq!(b, 3);
            },
            Message::ChangeColor(r, g, b) => {
                assert_eq!(g, 255); // Fill in with the correct value for green
                assert_eq!(b, 0);   // Fill in with the correct value for blue
            }
            _ => println!("no data in these variants"),
        }
    }
    fn main(){
        let msgs = [
            Message::Quit,
            Message::Move { x: 1, y: 3 },
            Message::ChangeColor(255, 255, 0),
        ];

        for msg in msgs {
            show_message(msg);
        }

        println!("Success!");
    }
}

#[test]
fn test814(){
    fn main(){
        let alphabets = ['a', 'E', 'Z', '0', 'x', '9', 'Y'];

        // Fill the blank with `matches!` to make the code work
        for ab in alphabets {
            // Assert only if the character is a letter
            if matches!(ab, 'a'..='z' | 'A'..='Z') {
                assert!(true);
            } else {
                continue;
            }
        }

        println!("Success!");
    }
}

#[test]
fn test815(){
    #[derive(PartialEq)]
    enum MyEnum {
        Foo,
        Bar,
    }
    fn main(){
        let mut count = 0;

        let v = vec![MyEnum::Foo, MyEnum::Bar, MyEnum::Foo];
        for e in v {
            if e == MyEnum::Foo { // Now this line works because PartialEq is derived
                count += 1;
            }
        }

        assert_eq!(count, 2);

        println!("Success!");
    }
}

#[test]
fn test816(){
    fn main(){
        let o = Some(7);

        // Use `if let` instead of `match`
        if let Some(i) = o {
            println!("This is a really long string and `{:?}`", i);
            println!("Success!");
        }
    }
}

#[test]
fn test817(){
    enum Foo {
        Bar(u8),
    }

    fn main(){
        let a = Foo::Bar(1);

        if let Foo::Bar(i) = a { // Fill in the blank
            println!("foobar holds the value: {}", i);
            println!("Success!");
        }
    }
}

#[test]
fn test818(){
    enum Foo {
        Bar,
        Baz,
        Qux(u32),
    }

    fn main(){
        let a = Foo::Qux(10);

        // Use `match` instead
        match a {
            Foo::Bar => {
                println!("match foo::bar");
            }
            Foo::Baz => {
                println!("match foo::baz");
            }
            _ => {
                println!("match others");
            }
        }
    }
}

#[test]
fn test819(){
    fn main(){
        let age = Some(30);
        if let Some(age) = age { // Create a new variable with the same name as previous `age`
            assert_eq!(age, 30); // Fix the assertion to compare with 30
        } // The new variable `age` goes out of scope here

        match age {
            // Match can also introduce a new shadowed variable
            Some(age) =>  println!("age is a new variable, its value is {}", age),
            _ => ()
        }
    }
}

#[test]
fn test821(){

    fn match_number(n: i32) {
        match n {
            // Match a single value
            1 => println!("One!"),
            // Fill in the blank with |, DON'T use .. or ..=
            2 | 3 | 4 | 5 => println!("match 2 -> 5"),
            // Match an inclusive range
            6..=10 => {
                println!("match 6 -> 10")
            },
            _ => {
                println!("match -infinite -> 0 or 11 -> +infinite")
            }
        }
    }

    fn main(){
        match_number(3);
    }
}

#[test]
fn test822(){
    struct Point {
        x: i32,
        y: i32,
    }

    fn main(){
        // Fill in the blank to let p match the second arm
        let p = Point { x: 4, y: 20 }; // Приклад значень

        match p {
            Point { x, y: 0 } => println!("On the x axis at {}", x),
            // Second arm
            Point { x: 0..=5, y: y @ (10 | 20 | 30) } => println!("On the y axis at {}", y),
            Point { x, y } => println!("On neither axis: ({}, {})", x, y),
        }
    }
}

#[test]
fn test823(){
    enum Message {
        Hello { id: i32 },
    }

    fn main(){
        let msg = Message::Hello { id: 5 };

        match msg {
            Message::Hello { id } if (3..=7).contains(&id) => {
                println!("Found an id in range [3, 7]: {}", id);
            }
            Message::Hello { id: newid @ (10 | 11 | 12) } => {
                println!("Found an id in another range [10, 12]: {}", newid);
            }
            Message::Hello { id } => {
                println!("Found some other id: {}", id);
            }
        }
    }
}


#[test]
fn test824(){
    fn main(){
        let num = Some(4);
        let split = 5;
        match num {
            Some(x) if x < split => assert!(x < split),
            Some(x) => assert!(x >= split),
            None => (),
        }

        println!("Success!");
    }
}

#[test]
fn test825(){
    fn main(){
        let numbers = (2, 4, 8, 16, 32, 64, 128, 256, 512, 1024, 2048);

        match numbers {
            (first, .., last) => {
                assert_eq!(first, 2);
                assert_eq!(last, 2048);
            }
        }

        println!("Success!");
    }
}

#[test]
fn test826(){
    fn main(){
        let mut v = String::from("hello,");
        let r = &mut v;

        match r {
            value => value.push_str(" world! 6")
        }

        println!("{}", v);
    }
}