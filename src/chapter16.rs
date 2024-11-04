use crate::main;

#[test]
fn test1611() {
    fn main(){
        let s1 = "hello";
        let s = format!("{}, {}!", s1, "world");
        assert_eq!(s, "hello, world!");
    }
    main()
}

#[test]
fn test1612() {

    fn main() {
        print!("hello world, ");
        print!("I am ");
        print!("Sunface!");
    }
    main()
}

#[test]
fn test1621() {
    // This structure cannot be printed either with `fmt::Display` or
    // with `fmt::Debug`.
    struct UnPrintable(i32);

    // To make this struct printable with `fmt::Debug`, we can derive the automatic implementations provided by Rust
    #[derive(Debug)]
    struct DebugPrintable(i32);

    // Fill in the blanks and Fix the errors
    #[derive(Debug)] // Deriving Debug for the Structure
    struct Structure(i32);

    fn main() {
        // Types in std and Rust have implemented the fmt::Debug trait
        println!("{} months in a year.", 12); // Fill in the blank with {}

        println!("Now {:?} will print!", Structure(3)); // Use {:?} to print the Structure with Debug
    }
    main()
}

#[test]
fn test1622() {
    struct Person {
        name: String,
        age: u8,
    }

    // Implementing a custom Debug trait for Person
    impl std::fmt::Debug for Person {
        fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
            // Custom formatting for Person
            write!(f, "Person {{\n    name: \"{}\",\n    age: {},\n}}", self.name, self.age)
        }
    }

    fn main() {
        let person = Person { name: "Sunface".to_string(), age: 18 };

        // Now it will output in the desired format
        println!("{:?}", person);
    }
    main()
}

#[test]
fn test1623() {
    // use std::fmt;
    //
    // struct Structure(i32);
    // impl fmt::Display for Structure {
    //     fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    //         write!(f, "{}", self.0)
    //     }
    // }
    //
    // impl fmt::Debug for Structure {
    //     fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    //         write!(f, "{}", self.0)
    //     }
    // }
    //
    // struct Deep(Structure);
    //
    // // Реалізуємо трейт fmt::Debug для Deep
    // impl fmt::Debug for Deep {
    //     fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    //         write!(f, "{}", self.0)
    //     }
    // }
    // #[test]
    // fn main() {
    //     println!("Now {:?} will print!", Deep(Structure(7)));
    // }
    // main()
}

#[test]
fn test1624() {
    // use std::fmt;
    // struct Point2D {
    //     x: f64,
    //     y: f64,
    // }
    //
    // impl fmt::Display for Point2D {
    //     fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    //         write!(f, "Display: {} + {}i", self.x, self.y)
    //     }
    // }
    //
    // impl fmt::Debug for Point2D {
    //     fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    //         write!(f, "Debug: Complex {{ real: {}, imag: {} }}", self.x, self.y)
    //     }
    // }
    //
    // #[test]
    // fn main() {
    //     let point = Point2D { x: 3.3, y: 7.2 };
    //
    //     assert_eq!(format!("{}", point), "Display: 3.3 + 7.2i");
    //     assert_eq!(format!("{:?}", point), "Debug: Complex { real: 3.3, imag: 7.2 }");
    //
    //     println!("Success! 4");
    // }
    // main()
}

#[test]
fn test1625() {
    use std::fmt;

    struct List(Vec<i32>);

    impl fmt::Display for List {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            // Отримуємо вектор
            let vec = &self.0;

            write!(f, "[")?; // Відкриваємо квадратні дужки

            // Проходимо по елементах вектора з їхніми індексами
            for (count, v) in vec.iter().enumerate() {
                // Для кожного елемента, крім першого, додаємо кому
                if count != 0 { write!(f, ", ")?; }
                write!(f, "{}: {}", count, v)?; // Виводимо індекс та значення
            }

            // Закриваємо квадратні дужки та повертаємо результат
            write!(f, "]")
        }
    }

    fn main() {
        let v = List(vec![1, 2, 3]);
        assert_eq!(format!("{}", v), "[0: 1, 1: 2, 2: 3]");
        println!("Success! 5");
    }
    main()
}

#[test]
fn test1631() {
    println!("{0}, this is {1}. {1}, this is {0}", "Alice", "Bob");
    assert_eq!(format!("{1}{0}", 1, 2), "21");
    assert_eq!(format!("{0}{1}{0}", 1, 2), "121");
    println!("Success!");
}

#[test]
fn test1632() {
    fn main() {
        println!("{argument}", argument = "test"); // => "test"

        /* Fill in the blanks */
        assert_eq!(format!("{name}{}", 1, name = "2"), "21");
        assert_eq!(format!("{a} {c} {b}", a = "a", b = 'b', c = 3), "a 3 b");

        /* Fix the error */
        println!("{0} {abc}", 2, abc = "def"); // Виправлення помилки

        println!("Success! 2");
    }
    main()
}

#[test]
fn test1633() {
    fn main(){ // The following two are padding with 5 spaces
        println!("Hello {:5}!", "x"); // =>  "Hello x    !"
        println!("Hello {:1$}!", "x", 5); // =>  "Hello x    !"

        /* Fill in the blanks */
        assert_eq!(format!("Hello {:1$}!", "x", 5), "Hello x    !");
        assert_eq!(format!("Hello {x:width$}!", x = "x", width = 5), "Hello x    !");

        println!("Success!");
    }
    main()
}

#[test]
fn test1634() {

    fn main() {
        // Left align
        println!("Hello {:<5}!", "x"); // => Hello x    !

        // Right align
        assert_eq!(format!("Hello {:>5}!", "x"), "Hello     x!"); // Correct

        // Center align
        assert_eq!(format!("Hello {:^7}!", "x"), "Hello   x  !"); // Correctly centered

        // Left align, pad with '&'
        assert_eq!(format!("Hello {:&<5}!", "x"), "Hello x&&&&!"); // Adjusted padding to match width of 5

        println!("Success!");
    }
}

#[test]
fn test1635() {
    fn main() {
        println!("Hello {:5}!", 5); // => Hello     5!
        println!("Hello {:+}!", 5); // =>  Hello +5!
        println!("Hello {:05}!", 5); // => Hello 00005!
        println!("Hello {:05}!", -5); // => Hello -0005!

        assert!(format!("{number:0>width$}", number=1, width=6) == "000001");

        println!("Success!");
    }
    main()
}

#[test]
fn test1636() {
    fn main() {
        let v = 3.1415926;

        println!("{:.1$}", v, 4); // same as {:.4} => 3.1416

        assert_eq!(format!("{:.2}", v), "3.14");
        assert_eq!(format!("{:+.2}", v), "+3.14");
        assert_eq!(format!("{:.0}", v), "3");

        println!("Success!");
    }
    main()
}

#[test]
fn test1637() {
    let v = 3.1415926;

    println!("{:.1$}", v, 4); // same as {:.4} => 3.1416

    assert_eq!(format!("{:.2}", v), "3.14");
    assert_eq!(format!("{:+.2}", v), "+3.14");
    assert_eq!(format!("{:.0}", v), "3");

    println!("Success!");
}

#[test]
fn test1638() {
    fn get_person() -> String {
        String::from("sunface")
    }

    fn get_format() -> (usize, usize) {
        (4, 1)
    }

    fn main(){
        let person = get_person();
        println!("Hello, {person}!");

        let (width, precision) = get_format();
        let scores = [("sunface", 99.12), ("jack", 60.34)];

        for (name, score) in scores {
            println!("{name}: {:.1$}", score, precision);
        }
    }
    main()
}