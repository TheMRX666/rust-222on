#[test]
fn test1711() {
    let i = 3;
    {
        let borrow1 = &i;
        println!("borrow1: {}", borrow1);
    }
    {
        let borrow2 = &i;
        println!("borrow2: {}", borrow2);
    }
}

#[test]
fn test1712() {
            let x = 5;
            let r;
            {
                r = &x;
            }
            println!("r: {}", r);
}

#[test]
fn test1713() {
    fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
        if x.len() > y.len() {
            x
        } else {
            y
        }
    }
        let string1 = String::from("long string");
        let string2 = String::from("short");
        let result = longest(&string1, &string2);
        println!("The longest string is: {}", result);
}

#[test]
fn test1714() {
    fn valid_output<'a>() -> String {
        String::from("foo")
    }
     let result = valid_output();
        println!("{}", result);
}

#[test]
fn test1715() {
    fn print_refs<'a, 'b>(x: &'a i32, y: &'b i32) {
        println!("x is {} and y is {}", x, y);
    }
    fn successful_return<'a>() -> i32 {
        let x = 12;
        x
    }
        let (four, nine) = (4, 9);
        print_refs(&four, &nine);

        let result = successful_return();
        println!("Result is {}", result);
}

#[test]
fn test1716() {
    #[derive(Debug)]
    struct Borrowed<'a>(&'a i32);
    #[derive(Debug)]
    struct NamedBorrowed<'a> {
        x: &'a i32,
        y: &'a i32,
    }
    #[derive(Debug)]
    enum Either<'a> {
        Num(i32),
        Ref(&'a i32),
    }
        let x = 18;
        let y = 15;

        let single = Borrowed(&x);
        let double = NamedBorrowed { x: &x, y: &y };
        let reference = Either::Ref(&x);
        let number    = Either::Num(y);

        println!("x is borrowed in {:?}", single);
        println!("x and y are borrowed in {:?}", double);
        println!("x is borrowed in {:?}", reference);
        println!("y is *not* borrowed in {:?}", number);
}

#[test]
fn test1717() {
    #[derive(Debug)]
    struct NoCopyType {}

    #[derive(Debug)]
    struct Example<'a> {
        a: &'a u32,
        b: Box<NoCopyType>,
    }
        let var_a = 35;
        let example: Example<'_>;

        {
            let var_b = NoCopyType {};

            example = Example { a: &var_a, b: Box::new(var_b) };
        }
        println!("(Success!) {:?}", example);
}

#[test]
fn test1718() {
    #[derive(Debug)]
    struct NoCopyType {}
    #[derive(Debug)]
    #[allow(dead_code)]
    struct Example<'a, 'b> {
        a: &'a u32,
        b: &'b NoCopyType,
    }
    fn fix_me<'a, 'b>(foo: &'a Example<'_, 'b>) -> &'b NoCopyType {
        foo.b
    }
        let no_copy = NoCopyType {};
        let example = Example { a: &1, b: &no_copy };

        let result = fix_me(&example);
        println!("Success! Reference to b: {:?}", result);
}

#[test]
fn test1719() {
    struct ImportantExcerpt<'a> {
        part: &'a str,
    }

    impl<'a> ImportantExcerpt<'a> {
        fn level(&self) -> i32 {
            3
        }
    }
        let text = String::from("This is an important excerpt.");
        let excerpt = ImportantExcerpt {
            part: &text,
        };
        println!("Level: {}", excerpt.level());
}

#[test]
fn test17110() {
    fn input(x: &i32) {
        println!("`annotated_input`: {}", x);
    }
    fn pass(x: &i32) -> &i32 { x }
    fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
        if x.len() > y.len() {
            x
        } else {
            y
        }
    }
    struct Owner(i32);
    impl Owner {
        fn add_one(&mut self) {
            self.0 += 1;
        }
        fn print(&self) {
            println!("`print`: {}", self.0);
        }
    }
    struct Person<'a> {
        age: u8,
        name: &'a str,
    }
    enum Either<'a> {
        Num(i32),
        Ref(&'a i32),
    }
}

#[test]
fn test1721() {
        let v: &'static str = "hello";
        need_static(v);
        println!("Success!")
    }
    fn need_static(r : &'static str) {
        assert_eq!(r, "hello");
}

#[test]
fn test1722() {
    #[derive(Debug)]
    struct Config {
        a: String,
        b: String,
    }
    static mut config: Option<&'static mut Config> = None;
    fn init() -> Option<&'static mut Config> {
        let config_box = Box::new(Config {
            a: "A".to_string(),
            b: "B".to_string(),
        });
        Some(Box::leak(config_box))
    }
     unsafe {
            config = init();
            println!("{:?}",config)
        }
}

#[test]
fn test1723() {
        let static_string: &'static str = "I'm in read-only memory";
        println!("static_string: {}", static_string);

        println!("static_string inside block: {}", static_string);

        println!("static_string reference remains alive: {}", static_string);
}

#[test]
fn test1724() {
    static NUM: i32 = 18;

    fn coerce_static<'a>(_: &'a i32) -> &'a i32 {
        &NUM
    }

            let lifetime_num = 9;

            let coerced_static = coerce_static(&lifetime_num);

            println!("coerced_static: {}", coerced_static);

        println!("NUM: {} stays accessible!", NUM);
}

#[test]
fn test1725() {
    use std::fmt::Debug;

    fn print_it<T: Debug + 'static>( input: T) {
        println!( "'static value passed in is: {:?}", input );
    }

    fn print_it1( input: impl Debug + 'static ) {
        println!( "'static value passed in is: {:?}", input );
    }


    fn print_it2<T: Debug + 'static>( input: &T) {
        println!( "'static value passed in is: {:?}", input );
    }

        let i = 5;
        print_it(i);

        print_it2(&i);
}

#[test]
fn test1726() {
    use std::fmt::Display;
     let mut string = "First".to_owned();

        string.push_str(string.to_uppercase().as_str());
        print_a(&string);
        print_b(&string);
        print_e(&string);
        print_f(&string);

    fn print_a<T: Display + 'static>(t: &T) {
        println!("{}", t);
    }

    fn print_b<T>(t: &T)
    where
        T: Display + 'static,
    {
        println!("{}", t);
    }

    fn print_c(t: &dyn Display) {
        println!("{}", t)
    }

    fn print_d(t: &impl Display) {
        println!("{}", t)
    }

    fn print_e(t: &(dyn Display + 'static)) {
        println!("{}", t)
    }

    fn print_f(t: &(impl Display + 'static)) {
        println!("{}", t)
    }

    fn print_g(t: &String) {
        println!("{}", t);
    }
}

#[test]
fn test1731() {
    struct DoubleRef<'a, 'b, T>
    where
        'a: 'b,
    {

        r: &'a T,
        s: &'b T,
    }
        println!("Success!")
}

#[test]
fn test1732() {
    struct ImportantExcerpt<'a> {
        part: &'a str,
    }
    impl<'a> ImportantExcerpt<'a> {
        fn announce_and_return_part<'b>(&'a self, announcement: &'b str) -> &'b str
        where
            'a: 'b,
        {
            println!("Attention please: {}", announcement);
            announcement
        }
    }
        println!("Success!")
}

#[test]
fn test1733() {
    fn f<'a, 'b>(x: &'a i32, y: &'b mut i32) {
        *y = *x;
        let r: &'a i32 = &0;
    }
        let a = 42;
        let mut b = 0;
        f(&a, &mut b);
        println!("Success! b = {}", b);
}

#[test]
fn test1734() {
    fn call_on_ref_zero<F>(f: F)
    where
        F: for<'a> Fn(&'a i32),
    {
        let zero = 0;
        f(&zero);
    }
     call_on_ref_zero(|x| {
            println!("Value: {}", x);
        });

        println!("Success!");
}

#[test]
fn test1735() {
        let mut data = 10;

        let ref1 = &mut data;
        *ref1 +=1;

        let ref2 = &mut data;
        *ref2 +=2;

        println!("{}", data);
}

#[test]
fn test1736() {
    struct Interface<'a> {
        manager: &'a Manager<'a>
    }
    impl<'a> Interface<'a> {
        pub fn noop(self) {
            println!("interface consumed");
        }
    }
    struct Manager<'a> {
        text: &'a str
    }
    struct List<'a> {
        manager: Manager<'a>,
    }
    impl<'a> List<'a> {
        pub fn get_interface(&self) -> Interface {
            Interface {
                manager: &self.manager
            }
        }
    }
     let list = List {
            manager: Manager {
                text: "hello"
            }
        };

        list.get_interface().noop();

        println!("Interface should be dropped here and the borrow released");

        use_list(&list);
    fn use_list(list: &List) {
        println!("{}", list.manager.text);
    }

}