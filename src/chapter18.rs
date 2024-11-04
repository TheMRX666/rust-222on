#[test]
fn test1811() {
    let color = String::from("green");

    let print = || println!("`color`: {}", color);

    print();
    print();

    let _reborrow = &color;

    println!("{}", color);
}

#[test]
fn test1812() {
    fn main(){
        let mut count = 0;

        let mut inc = || {
            let mut local_count = count; // Створюємо копію значення
            local_count += 1;
            println!("`count`: {}", local_count);
        };

        inc();
        inc();

        assert_eq!(count, 0);
    }
    main();
}

#[test]
fn test1813() {
    fn take<T>(_v: T) {}

    fn main(){
        let movable = Box::new(3);

        let consume = || {
            println!("`movable`: {:?}", movable);
            take(movable.clone());
        };

        consume();
        consume();
    }
    main();
}

#[test]
fn test1815() {
    fn fn_once<F>(func: F)
    where
        F: Fn(usize) -> bool,
    {
        println!("{}", func(3));
        println!("{}", func(4));
    }

    fn main(){
        let x = vec![1, 2, 3];
        fn_once(|z| z == x.len());
    }
}

#[test]
fn test1816() {
    fn exec<'a, F: FnMut(&'a str)>(mut f: F) {
        f("hello")
    }

    fn main(){
        let mut s = String::new();

        let update_string = |str| s.push_str(str);

        exec(update_string);

        println!("{:?}", s);
    }
    main()
}

#[test]
fn test1817() {
    use std::mem;
    fn apply<F>(f: F)
    where
    // The closure takes no input and returns nothing.
        F: FnOnce() {

        f();
    }

    // A function which takes a closure and returns an `i32`.
    fn apply_to_3<F>(f: F) -> i32
    where
    // The closure takes an `i32` and returns an `i32`.
        F: Fn(i32) -> i32 {

        f(3)
    }

    fn main() {
        let greeting = "hello";
        // A non-copy type.
        // `to_owned` creates owned data from borrowed one
        let mut farewell = "goodbye".to_owned();

        // Capture 2 variables: `greeting` by reference and
        // `farewell` by value.
        let diary = || {
            // `greeting` is by reference: requires `Fn`.
            println!("I said {}.", greeting);

            // Mutation forces `farewell` to be captured by
            // mutable reference. Now requires `FnMut`.
            farewell.push_str("!!!");
            println!("Then I screamed {}.", farewell);
            println!("Now I can sleep. zzzzz");

            // Manually calling drop forces `farewell` to
            // be captured by value. Now requires `FnOnce`.
            mem::drop(farewell);
        };

        // Call the function which applies the closure.
        apply(diary);

        // `double` satisfies `apply_to_3`'s trait bound
        let double = |x| 2 * x;

        println!("3 doubled: {}", apply_to_3(double));
    }
    main()
}

#[test]
fn test1818(){
    fn exec<'a, F: FnMut(&'a str) -> String>(mut f: F) {
        f("hello");
    }

    fn main(){
        let mut s = String::new();

        let update_string = |str: &str| -> String { s.push_str(str); s.clone() };

        exec(update_string);
    }
    main()
}

#[test]
fn test1819() {
    fn call_me<F>(f: F)
    where
        F: Fn(),
    {
        f();
    }

    fn function() {
        println!("I'm a function!");
    }

    fn main(){
        let closure = || println!("I'm a closure!");

        call_me(closure);
        call_me(function);
    }
}

#[test]
fn test18110() {
    fn create_fn() -> Box<dyn Fn(i32) -> i32> {
        let num = 5;

        Box::new(move |x| x + num)
    }

    fn main(){
        let fn_plain = create_fn();
        println!("{}", fn_plain(1));
    }
}

#[test]
fn test18111() {
    fn factory(x: i32) -> Box<dyn Fn(i32) -> i32> {
        let num = 5;

        if x > 1 {
            Box::new(move |y| y + num)
        } else {
            Box::new(move |y| y + num)
        }
    }

    fn main(){
        let f = factory(2);
        println!("{}", f(3));
    }
    main()
}

#[test]
fn test1821() {
    fn main(){
        let arr = [0; 10];
        for val in arr.iter() {
            println!("{}", val);
        }
    }
    main()
}

#[test]
fn test1822() {
    let mut v = Vec::new();
    for n in 0..100 {
        v.push(n);
    }

    assert_eq!(v.len(), 100);
}

#[test]
fn test1823() {
    let mut v1 = vec![1, 2].into_iter();

    assert_eq!(v1.next(), Some(1));
    assert_eq!(v1.next(), Some(2));
    assert_eq!(v1.next(), None);
}

#[test]
fn test1824() {
    fn main(){
        let arr = vec![0; 10];
        for &i in &arr {
            println!("{}", i);
        }

        println!("{:?}", arr);
    }
    main()
}

#[test]
fn test1825() {
    fn main(){
        let mut names = vec!["Bob", "Frank", "Ferris"];

        for name in names.iter_mut() {
            *name = match *name {
                "Ferris" => "There is a rustacean among us!",
                _ => "Hello",
            }
        }

        println!("names: {:?}", names);
    }
    main()
}

#[test]
fn test1826() {
    fn main(){
        let mut values = vec![1, 2, 3];
        let mut values_iter = values.iter_mut();

        if let Some(v) = values_iter.next() {
            *v = 0;
        }

        assert_eq!(values, vec![0, 2, 3]);
    }
}

#[test]
fn test1827() {
    struct Fibonacci {
        curr: u32,
        next: u32,
    }

    // Implement `Iterator` for `Fibonacci`.
    impl Iterator for Fibonacci {
        // We can refer to this type using Self::Item
        type Item = u32; // Встановлюємо Item як u32

        /* Implement next method */
        fn next(&mut self) -> Option<Self::Item> {
            // Повертаємо поточне число
            let curr = self.curr;
            // Оновлюємо значення curr і next
            self.curr = self.next;
            self.next = curr + self.next;

            // Повертаємо curr
            if curr == 0 {
                Some(1) // Повертаємо 1 при першому виклику
            } else {
                Some(curr) // Для всіх наступних викликів повертаємо curr
            }
        }
    }

    // Returns a Fibonacci sequence generator
    fn fibonacci() -> Fibonacci {
        Fibonacci { curr: 0, next: 1 }
    }

    fn main(){
        let mut fib = fibonacci();
            assert_eq!(fib.next(), Some(1));
            assert_eq!(fib.next(), Some(1));
            assert_eq!(fib.next(), Some(2));
            assert_eq!(fib.next(), Some(3));
            assert_eq!(fib.next(), Some(5));
    }
}


#[test]
fn test1828() {
    let v1 = vec![1, 2, 3];

    let total: i32 = v1.iter().sum();

    assert_eq!(total, 6);

    println!("{:?}", v1);
}

#[test]
fn test1829() {
    use std::collections::HashMap;

    fn main(){
        let names = [("sunface", 18), ("sunfei", 18)];
            let folks: HashMap<_, _> = names.iter().cloned().collect(); // Використовуємо `iter().cloned()` для копіювання

            println!("{:?}", folks);

            let v1: Vec<i32> = vec![1, 2, 3];

            // Явно вказуємо тип для collect
            let v2: Vec<i32> = v1.iter().cloned().collect(); // Використовуємо `cloned()` для отримання значень

            assert_eq!(v2, vec![1, 2, 3]);
    }
    main()
}

#[test]
fn test18210() {
    let v1: Vec<i32> = vec![1, 2, 3];

    let v2: Vec<_> = v1.iter().map(|&x| x + 1).collect();

    assert_eq!(v2, vec![2, 3, 4]);
}

#[test]
fn test18211() {
    use std::collections::HashMap;

    fn main(){
        let names = ["sunface", "sunfei"];
        let ages = [18, 18];


        let folks: HashMap<_, _> = names.iter().zip(ages.iter()).collect();

        println!("{:?}", folks);
    }
    main()
}

#[test]
fn test18212() {
    #[derive(PartialEq, Debug)]
    struct Shoe {
        size: u32,
        style: String,
    }

    fn shoes_in_size(shoes: Vec<Shoe>, shoe_size: u32) -> Vec<Shoe> {
        shoes.into_iter().filter(|shoe| shoe.size == shoe_size).collect()
    }

    fn main(){
        let shoes = vec![
            Shoe {
                size: 10,
                style: String::from("sneaker"),
            },
            Shoe {
                size: 13,
                style: String::from("sandal"),
            },
            Shoe {
                size: 10,
                style: String::from("boot"),
            },
        ];

        let in_my_size = shoes_in_size(shoes, 10);

        assert_eq!(
            in_my_size,
            vec![
                Shoe {
                    size: 10,
                    style: String::from("sneaker")
                },
                Shoe {
                    size: 10,
                    style: String::from("boot")
                },
            ]
        );
    }

    main()
}