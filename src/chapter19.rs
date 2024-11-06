#[test]
fn test191() {
    use std::fmt;
    struct Wrapper(Vec<String>);
    impl fmt::Display for Wrapper {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            write!(f, "[{}]", self.0.join(", "))
        }
    }
        let w = Wrapper(vec![String::from("hello"), String::from("world")]);
        println!("w = {}", w);
}

#[test]
fn test192() {
    struct Meters(u32);
    impl Meters {
        fn pow(&self, exp: u32) -> u32 {
            self.0.pow(exp)
        }
    }
     let i: u32 = 2;
        assert_eq!(i.pow(2), 4);

        let n = Meters(i);
        assert_eq!(n.pow(2), 4);
}

#[test]
fn test193() {
    struct Years(i64);
    struct Days(i64);

    impl Years {
        pub fn to_days(&self) -> Days {
            Days(self.0 * 365)
        }
    }
    impl Days {
        pub fn to_years(&self) -> Years {
            Years(self.0 / 365)
        }
    }
    fn old_enough_years(age: &Years) -> bool {
        age.0 >= 18
    }
    fn old_enough_days(age: &Days) -> bool {
        age.to_years().0 >= 18
    }
        let age = Years(5);
        let age_days = age.to_days();
        println!("Old enough {}", old_enough_years(&age));
        println!("Old enough {}", old_enough_days(&age_days));
}

#[test]
fn test194() {
    use std::ops::Add;
    use std::fmt;
    struct Meters(u32);
    impl fmt::Display for Meters {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            write!(f, "There are still {} meters left", self.0)
        }
    }
    impl Add for Meters {
        type Output = Self;

        fn add(self, other: Meters) -> Self {
            Self(self.0 + other.0)
        }
    }
    fn calculate_distance(a: Meters, b: Meters) -> Meters {
        a + b
    }
        let d = calculate_distance(Meters(10), Meters(20));
        assert_eq!(format!("{}", d), "There are still 30 meters left");
        println!("{}", d);
}

#[test]
fn test195() {
    enum VeryVerboseEnumOfThingsToDoWithNumbers {
        Add,
        Subtract,
    }
    type Operations = VeryVerboseEnumOfThingsToDoWithNumbers;

        let x = Operations::Add;
}

#[test]
fn test196() {
    enum VeryVerboseEnumOfThingsToDoWithNumbers {
        Add,
        Subtract,
    }

    impl VeryVerboseEnumOfThingsToDoWithNumbers {
        fn run(&self, x: i32, y: i32) -> i32 {
            match self {
                VeryVerboseEnumOfThingsToDoWithNumbers::Add => x + y,
                VeryVerboseEnumOfThingsToDoWithNumbers::Subtract => x - y,
            }
        }
    }
        let operation_add = VeryVerboseEnumOfThingsToDoWithNumbers::Add;
        let operation_subtract = VeryVerboseEnumOfThingsToDoWithNumbers::Subtract;

        let result_add = operation_add.run(2, 2);
        let result_subtract = operation_subtract.run(2, 2);

        println!("2 + 2 = {}", result_add);
        println!("2 - 2 = {}", result_subtract);
}

#[test]
fn test197() {
    fn my_function<const N: usize>() -> [u32; N] {
        [123; N]
    }
        let arr = my_function::<5>();
        println!("{:?}", arr);
}

#[test]
fn test198() {
    let s: &str = "Hello there!";
    let arr: &[u8] = &[1, 2, 3];
}

#[test]
fn test199() {
    use std::fmt::Display;
    fn foobar(thing: Box<dyn Display>) {
        println!("{}", thing);
    }
     let my_string = String::from("Hello, world!");
        foobar(Box::new(my_string));
}
