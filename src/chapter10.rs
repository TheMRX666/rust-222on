#[test]
fn test1011() {
    struct A;          // Concrete type `A`.
    struct S(A);       // Concrete type `S`.
    struct SGen<T>(T); // Generic type `SGen`.

    fn reg_fn(_s: S) {}

    fn gen_spec_t(_s: SGen<A>) {}

    fn gen_spec_i32(_s: SGen<i32>) {}

    fn generic<T>(_s: SGen<T>) {}
    fn main(){
        // Using the non-generic functions
        reg_fn(S(A));          // Concrete type.
        gen_spec_t(SGen(A));   // Implicitly specified type parameter `A`.
        gen_spec_i32(SGen(42)); // Implicitly specified type parameter `i32`.

        // Explicitly specified type parameter `char` to `generic()`.
        generic::<char>(SGen('a'));

        // Implicitly specified type parameter `char` to `generic()`.
        generic(SGen('b'));

        println!("Success!");
    }
    main()
}

#[test]
fn test1012() {
    fn sum<T>(a: T, b: T) -> T
    where
        T: std::ops::Add<Output = T> + Copy,
    {
        a + b
    }

    fn main(){
        assert_eq!(5, sum(2i8, 3i8));
        assert_eq!(50, sum(20, 30));
        assert_eq!(2.46, sum(1.23, 1.23));

        println!("Success!");
    }
    main()
}

#[test]
fn test1013() {
    #[derive(Debug)]
    struct Point3<T> {
        x: T,
        y: T,
    }

    fn main(){
        let integer = Point3 { x: 5, y: 10 };
        let float = Point3 { x: 1.0, y: 4.0 };

        println!("Success!");
    }
    main()
}

#[test]
fn test1014() {
    struct Point4<X, Y> {
        x: X,
        y: Y,
    }

    fn main() {
        // DON'T modify this code.
        let p = Point4 { x: 5, y: "hello".to_string() };

        println!("Success!");
    }
    main()
}

#[test]
fn test1015() {
    struct Val<T> {
        val: T,
    }

    impl Val<f64> {
        fn value(&self) -> &f64 {
            &self.val
        }
    }

    impl Val<String> {
        fn value(&self) -> &String {
            &self.val
        }
    }

    fn main(){
        let x = Val { val: 5.0 };
        let y = Val { val: "hello".to_string() };
        println!("{}, {}", x.value(), y.value());
    }
    main()
}

#[test]
fn test1016() {
    struct Point6<T, U> {
        x: T,
        y: U,
    }

    impl<T, U> Point6<T, U> {
        // Implement mixup to make it work, DON'T modify other code.
        fn mixup<V, W>(self, other: Point6<V, W>) -> Point6<T, W> {
            Point6 {
                x: self.x,
                y: other.y,
            }
        }
    }

    fn main(){
        let p1 = Point6 { x: 5, y: 10 };
        let p2 = Point6 { x: "Hello", y: '中' };

        let p3 = p1.mixup(p2);

        assert_eq!(p3.x, 5);
        assert_eq!(p3.y, '中');

        println!("Success!");
    }
    main()
}

#[test]
fn test1017() {
    struct Point7<T> {
        x: T,
        y: T,
    }

    impl Point7<f32> {
        fn distance_from_origin(&self) -> f32 {
            (self.x.powi(2) + self.y.powi(2)).sqrt()
        }
    }

    fn main(){
        // Use f32 for Point
        let p = Point7 { x: 5.0f32, y: 10.0f32 }; // Use f32 instead of i32
        println!("{}", p.distance_from_origin());
    }
    main()
}

#[test]
fn test1021() {
    struct Array<T, const N: usize> {
        data: [T; N],
    }

    fn main() {
        let arrays: [&Array<i32, 3>; 2] = [
            &Array {
                data: [1, 2, 3],
            },
            &Array {
                data: [4, 5, 6],
            },
        ];

        let arrays_float: [&Array<f64, 3>; 1] = [
            &Array {
                data: [1.0, 2.0, 3.0],
            },
        ];

        println!("Success!");
    }
    main()
}

#[test]
fn test1022() {
    fn print_array<T: std::fmt::Debug>(arr: &[T]) {
        println!("{:?}", arr);
    }

    fn main() {
        let arr = [1, 2, 3];
        print_array(&arr);

        let arr = ["hello", "world 2"];
        print_array(&arr);
    }
    main()
}

#[test]
fn test1031() {

    trait Hello {
        fn say_hi(&self) -> String {
            String::from("hi")
        }

        fn say_something(&self) -> String;
    }

    struct Student {}

    impl Hello for Student {
        fn say_something(&self) -> String {
            String::from("I'm a good student")
        }
    }

    struct Teacher {}

    impl Hello for Teacher {
        fn say_hi(&self) -> String {
            String::from("Hi, I'm your new teacher")
        }

        fn say_something(&self) -> String {
            String::from("I'm not a bad teacher")
        }
    }

    fn main(){
        let s = Student {};
        assert_eq!(s.say_hi(), "hi");
        assert_eq!(s.say_something(), "I'm a good student");

        let t = Teacher {};
        assert_eq!(t.say_hi(), "Hi, I'm your new teacher");
        assert_eq!(t.say_something(), "I'm not a bad teacher");

        println!("Success!");
    }
    main()
}

#[test]
fn test1032() {

    // `Centimeters`, a tuple struct that can be compared
    #[derive(PartialEq, PartialOrd)]
    struct Centimeters(f64);

    // `Inches`, a tuple struct that can be printed
    #[derive(Debug)]
    struct Inches(i32);

    impl Inches {
        fn to_centimeters(&self) -> Centimeters {
            let &Inches(inches) = self;

            Centimeters(inches as f64 * 2.54)
        }
    }

    // ADD some attributes to make the code work!
    // DON'T modify other code!
    #[derive(Debug, PartialEq, PartialOrd)] // Added these attributes
    struct Seconds(i32);

    fn main(){
        let _one_second = Seconds(1);

        println!("One second looks like: {:?}", _one_second);
        let _this_is_true = (_one_second == _one_second);
        let _this_is_false = (_one_second > _one_second);

        let foot = Inches(12);

        println!("One foot equals {:?}", foot);

        let meter = Centimeters(100.0);

        let cmp =
            if foot.to_centimeters() < meter {
                "smaller"
            } else {
                "bigger"
            };

        println!("One foot is {} than one meter.", cmp);
    }
    main()
}


#[test]
fn test1033() {
    use std::ops::Mul;

    // Implement fn multiply to make the code work.
    fn multiply<T: Mul<Output = T>>(a: T, b: T) -> T {
        a * b
    }

    fn main(){
        assert_eq!(6, multiply(2u8, 3u8));
        assert_eq!(5.0, multiply(1.0, 5.0));

        println!("Success!");
    }
    main()
}

#[test]
fn test1034() {
    use std::ops;

    struct Foo;
    struct Bar;

    #[derive(Debug)] // Derive Debug for FooBar
    struct FooBar;

    #[derive(Debug)] // Derive Debug for BarFoo
    struct BarFoo;

    // The `std::ops::Add` trait is used to specify the functionality of `+`.
    // Here, we make `Add<Bar>` - the trait for addition with a RHS of type `Bar`.
    // The following block implements the operation: Foo + Bar = FooBar
    impl ops::Add<Bar> for Foo {
        type Output = FooBar;

        fn add(self, _rhs: Bar) -> FooBar {
            FooBar
        }
    }

    // Implement Sub for Foo
    impl ops::Sub<Bar> for Foo {
        type Output = BarFoo;

        fn sub(self, _rhs: Bar) -> BarFoo {
            BarFoo
        }
    }

    // Implement Sub for Bar
    impl ops::Sub<Foo> for Bar {
        type Output = BarFoo;

        fn sub(self, _rhs: Foo) -> BarFoo {
            BarFoo
        }
    }

    // Derive PartialEq for FooBar to make it comparable
    impl PartialEq for FooBar {
        fn eq(&self, _other: &FooBar) -> bool {
            true // Always return true, as they are the same type
        }
    }

    impl PartialEq for BarFoo {
        fn eq(&self, _other: &BarFoo) -> bool {
            true // Always return true, as they are the same type
        }
    }

    fn main(){
        // DON'T modify the code below.
        // You need to derive some trait for FooBar to make it comparable.
        assert_eq!(Foo + Bar, FooBar);
        assert_eq!(Foo - Bar, BarFoo);

        println!("Success!");
    }
    main()
}

#[test]
fn test1035() {
    trait Summary {
        fn summarize(&self) -> String;
    }

    #[derive(Debug)]
    struct Post {
        title: String,
        author: String,
        content: String,
    }

    impl Summary for Post {
        fn summarize(&self) -> String {
            format!("The author of post {} is {}", self.title, self.author)
        }
    }

    #[derive(Debug)]
    struct Weibo {
        username: String,
        content: String,
    }

    impl Summary for Weibo {
        fn summarize(&self) -> String {
            format!("{} published a weibo {}", self.username, self.content)
        }
    }

    // Implement `fn summary` below.
    fn summary<S: Summary>(item: &S) { // Change to accept a reference
        println!("{}", item.summarize());
    }

    fn main() {
        let post = Post {
            title: "Popular Rust".to_string(),
            author: "Sunface".to_string(),
            content: "Rust is awesome!".to_string(),
        };
        let weibo = Weibo {
            username: "sunface".to_string(),
            content: "Weibo seems to be worse than Tweet".to_string(),
        };

        summary(&post); // Pass a reference to `post`
        summary(&weibo); // Pass a reference to `weibo`

        println!("{:?}", post); // Now you can use `post` again
        println!("{:?}", weibo); // You can use `weibo` again
    }
    main()
}


#[test]
fn test1036() {
    struct Sheep {}
    struct Cow {}

    trait Animal {
        fn noise(&self) -> String;
    }

    impl Animal for Sheep {
        fn noise(&self) -> String {
            "baaaaah!".to_string()
        }
    }

    impl Animal for Cow {
        fn noise(&self) -> String {
            "moooooo!".to_string()
        }
    }

    // Returns a Box<dyn Animal> to use trait objects
    fn random_animal(random_number: f64) -> Box<dyn Animal> {
        if random_number < 0.5 {
            Box::new(Sheep {})
        } else {
            Box::new(Cow {})
        }
    }

    fn main() {
        let random_number = 0.234;
        let animal = random_animal(random_number);
        println!("You've randomly chosen an animal, and it says {}", animal.noise());
    }
    main()
}


#[test]
fn test1037() {
    use std::ops::Add;

    fn sum<T>(x: T, y: T) -> T
    where
        T: Add<Output = T>, // Вказуємо, що T має реалізовувати трейт Add
    {
        x + y
    }

    fn main(){
        assert_eq!(sum(1, 2), 3);
    }
    main()
}

#[test]
fn test1038() {
    use std::fmt;

    #[derive(Debug)] // Automatically derive Debug
    struct Unit(i32);

    impl PartialOrd for Unit { // Implement PartialOrd
        fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
            self.0.partial_cmp(&other.0) // Compare the inner values
        }
    }

    impl PartialEq for Unit { // Implement PartialEq
        fn eq(&self, other: &Self) -> bool {
            self.0 == other.0 // Compare the inner values
        }
    }

    struct Pair<T> {
        x: T,
        y: T,
    }

    impl<T> Pair<T> {
        fn new(x: T, y: T) -> Self {
            Self { x, y }
        }
    }

    impl<T: fmt::Debug + PartialOrd> Pair<T> {
        fn cmp_display(&self) {
            if self.x >= self.y {
                println!("The largest member is x = {:?}", self.x);
            } else {
                println!("The largest member is y = {:?}", self.y);
            }
        }
    }

    fn main() {
        let pair = Pair {
            x: Unit(1),
            y: Unit(3),
        };

        pair.cmp_display();
    }
    main()
}

#[test]
fn test1039() {
    use std::collections::HashMap;
    use std::hash::Hash;
    fn example1() {
        struct Cacher<T>
        where
            T: Fn(u32) -> u32,
        {
            calculation: T,
            values: HashMap<u32, u32>,
        }

        impl<T> Cacher<T>
        where
            T: Fn(u32) -> u32,
        {
            fn new(calculation: T) -> Cacher<T> {
                Cacher {
                    calculation,
                    values: HashMap::new(),
                }
            }

            fn value(&mut self, arg: u32) -> u32 {
                if let Some(&v) = self.values.get(&arg) {
                    v
                } else {
                    let v = (self.calculation)(arg);
                    self.values.insert(arg, v);
                    v
                }
            }
        }

        let mut cacher = Cacher::new(|x| x + 1);
        assert_eq!(cacher.value(10), 11); // 10 + 1 = 11
        assert_eq!(cacher.value(15), 16); // 15 + 1 = 16
    }

    fn example2() {
        struct Cacher<T>
        where
            T: Fn(u32) -> u32,
        {
            calculation: T,
            values: HashMap<u32, u32>,
        }

        impl<T> Cacher<T>
        where
            T: Fn(u32) -> u32,
        {
            fn new(calculation: T) -> Cacher<T> {
                Cacher {
                    calculation,
                    values: HashMap::new(),
                }
            }

            fn value(&mut self, arg: u32) -> u32 {
                if let Some(&v) = self.values.get(&arg) {
                    v
                } else {
                    let v = (self.calculation)(arg);
                    self.values.insert(arg, v);
                    v
                }
            }
        }

        let mut cacher = Cacher::new(|x| x + 1);
        assert_eq!(cacher.value(20), 21);
        assert_eq!(cacher.value(25), 26);
    }

    fn main() {
        example1();
        example2();

        println!("Success!");
    }
    main()
}

#[test]
fn test1041(){
    trait Bird1 {
        fn quack(&self) -> String;
    }

    struct Duck1;
    impl Duck1 {
        fn swim(&self) {
            println!("Look, the duck is swimming");
        }
    }
    struct Swan1;
    impl Swan1 {
        fn fly(&self) {
            println!("Look, the duck... oh sorry, the swan is flying");
        }
    }

    impl Bird1 for Duck1 {
        fn quack(&self) -> String {
            "duck duck".to_string()
        }
    }

    impl Bird1 for Swan1 {
        fn quack(&self) -> String {
            "swan swan".to_string()
        }
    }

    fn hatch_a_bird(n: i32) -> Box<dyn Bird1> {
        if n == 2 {
            Box::new(Duck1)
        } else {
            Box::new(Swan1)
        }
    }
    fn main() {
        let duck = Duck1;
        duck.swim();

        let bird = hatch_a_bird(2);
        assert_eq!(bird.quack(), "duck duck");

        let bird = hatch_a_bird(1);
        assert_eq!(bird.quack(), "swan swan");

        println!("Success! 1");
    }
    main()
}

#[test]
fn test1042(){
    trait Bird {
        fn quack(&self);
    }

    struct Duck;
    impl Duck {
        fn fly(&self) {
            println!("Look, the duck is flying");
        }
    }

    struct Swan;
    impl Swan {
        fn fly(&self) {
            println!("Look, the duck.. oh sorry, the swan is flying");
        }
    }

    impl Bird for Duck {
        fn quack(&self) {
            println!("{}", "duck duck");
        }
    }

    impl Bird for Swan {
        fn quack(&self) {
            println!("{}", "swan swan");
        }
    }

    fn main() {
        let birds: Vec<Box<dyn Bird>> = vec![Box::new(Duck), Box::new(Swan)];

        for bird in birds {
            bird.quack();
        }
    }
    main()
}

#[test]
fn test1043(){
    trait Draw {
        fn draw(&self) -> String;
    }

    impl Draw for u8 {
        fn draw(&self) -> String {
            format!("u8: {}", *self)
        }
    }

    impl Draw for f64 {
        fn draw(&self) -> String {
            format!("f64: {}", *self)
        }
    }
    fn draw_with_box(x: Box<dyn Draw>) {
        println!("{}", x.draw());
    }

    fn draw_with_ref(x: &dyn Draw) {
        println!("{}", x.draw());
    }

    fn main() {
        let x = 1.1f64;
        let y = 8u8;

        // Draw x.
        draw_with_box(Box::new(x));
        // Draw y.
        draw_with_ref(&y);

        println!("Success!");
    }
    main()
}

#[test]
fn test1044(){
    trait Foo {
        fn method(&self) -> String;
    }

    impl Foo for u8 {
        fn method(&self) -> String { format!("u8: {}", *self) }
    }

    impl Foo for String {
        fn method(&self) -> String { format!("string: {}", *self) }
    }


    fn static_dispatch<T: Foo>(value: T) {
        println!("{}", value.method());
    }

    fn dynamic_dispatch(value: &dyn Foo) {
        println!("{}", value.method());
    }

    fn main(){
        let x = 5u8;
        let y = "Hello".to_string();

        static_dispatch(x);
        dynamic_dispatch(&y);

        println!("Success!");
    }
    main()
}

#[test]
fn test1045(){
    trait MyTrait {
        fn f(&self) -> Box<dyn MyTrait>;
    }

    impl MyTrait for u32 {
        fn f(&self) -> Box<dyn MyTrait> { Box::new(42) }
    }

    impl MyTrait for String {
        fn f(&self) -> Box<dyn MyTrait> { Box::new(self.clone()) }
    }

    fn my_function(x: Box<dyn MyTrait>) {
        x.f();
    }

    fn main() {
        my_function(Box::new(13_u32));
        my_function(Box::new(String::from("abc")));

        println!("Success!");
    }
    main()
}

#[test]
fn test1051() {
    struct Container(i32, i32);

    trait Contains<A, B> {
        fn contains(&self, _: &A, _: &B) -> bool;
        fn first(&self) -> i32;
        fn last(&self) -> i32;
    }

    impl Contains<i32, i32> for Container {
        fn contains(&self, number_1: &i32, number_2: &i32) -> bool {
            (&self.0 == number_1) && (&self.1 == number_2)
        }

        fn first(&self) -> i32 { self.0 }

        fn last(&self) -> i32 { self.1 }
    }

    fn difference<A, B, C: Contains<A, B>>(container: &C) -> i32 {
        container.last() - container.first()
    }

    fn main(){
        let number_1 = 3;
        let number_2 = 10;

        let container = Container(number_1, number_2);

        println!("Does container contain {} and {}: {}",
                 &number_1, &number_2,
                 container.contains(&number_1, &number_2));
        println!("First number: {}", container.first());
        println!("Last number: {}", container.last());

        println!("The difference is: {}", difference(&container));
    }
    main()
}

#[test]
fn test1052(){
    use std::ops::Sub;

    #[derive(Debug, PartialEq)]
    struct Point<T> {
        x: T,
        y: T,
    }

    impl<T> Sub for Point<T>
    where
        T: Sub<Output = T>,
    {
        type Output = Self;

        fn sub(self, other: Self) -> Self::Output {
            Point {
                x: self.x - other.x,
                y: self.y - other.y,
            }
        }
    }

    fn main(){
        assert_eq!(Point { x: 2, y: 3 } - Point { x: 1, y: 0 },
                   Point { x: 1, y: 3 });

        println!("Success!");
    }
    main()
}

#[test]
fn test1053(){
    trait Pilot {
        fn fly(&self) -> String;
    }

    trait Wizard {
        fn fly(&self) -> String;
    }

    struct Human;

    impl Pilot for Human {
        fn fly(&self) -> String {
            String::from("This is your captain speaking.")
        }
    }

    impl Wizard for Human {
        fn fly(&self) -> String {
            String::from("Up!")
        }
    }

    impl Human {
        fn fly(&self) -> String {
            String::from("*waving arms furiously*")
        }
    }

    fn main(){
        let person = Human;

        assert_eq!(Pilot::fly(&person), "This is your captain speaking.");
        assert_eq!(Wizard::fly(&person), "Up!");

        assert_eq!(person.fly(), "*waving arms furiously*");

        println!("Success!");
    }
    main()
}

#[test]
fn test1054(){
    trait Person {
        fn name(&self) -> String;
    }

    // Person is a supertrait of Student.
    // Implementing Student requires you to also impl Person.
    trait Student: Person {
        fn university(&self) -> String;
    }

    trait Programmer {
        fn fav_language(&self) -> String;
    }

    // CompSciStudent (computer science student) is a subtrait of both Programmer
    // and Student. Implementing CompSciStudent requires you to impl both supertraits.
    trait CompSciStudent: Programmer + Student {
        fn git_username(&self) -> String;
    }

    fn comp_sci_student_greeting(student: &dyn CompSciStudent) -> String {
        format!(
            "My name is {} and I attend {}. My favorite language is {}. My Git username is {}",
            student.name(),
            student.university(),
            student.fav_language(),
            student.git_username()
        )
    }

    struct CSStudent {
        name: String,
        university: String,
        fav_language: String,
        git_username: String,
    }

    // Implement the necessary traits for CSStudent to make the code work
    impl Person for CSStudent {
        fn name(&self) -> String {
            self.name.clone()
        }
    }

    impl Student for CSStudent {
        fn university(&self) -> String {
            self.university.clone()
        }
    }

    impl Programmer for CSStudent {
        fn fav_language(&self) -> String {
            self.fav_language.clone()
        }
    }

    impl CompSciStudent for CSStudent {
        fn git_username(&self) -> String {
            self.git_username.clone()
        }
    }


    fn main(){
        let student = CSStudent {
            name: "Sunfei".to_string(),
            university: "XXX".to_string(),
            fav_language: "Rust".to_string(),
            git_username: "sunface".to_string(),
        };

        // Pass the student to the function
        println!("{}", comp_sci_student_greeting(&student));
    }
    main()
}

#[test]
fn test1055(){
    use std::fmt;

    // DEFINE a newtype `Pretty` here
    struct Pretty(String);

    impl fmt::Display for Pretty {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            write!(f, "\"{}\"", self.0.clone() + ", world 5")
        }
    }

    fn main(){
        let w = Pretty("hello".to_string());
        println!("w = {}", w);
    }
    main()
}